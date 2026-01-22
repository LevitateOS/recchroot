//! recchroot - Enter chroot with proper bind mounts
//!
//! Like arch-chroot for Arch Linux - sets up /dev, /proc, /sys, /run, then chroots.
//! Does ONE thing: enter chroot properly. Cleans up on exit.
//!
//! Usage:
//!   recchroot /mnt              # Interactive shell
//!   recchroot /mnt passwd       # Run single command
//!
//! This is NOT an installer. This enters chroot. That's it.

use anyhow::{bail, Context, Result};
use clap::Parser;
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sys::signal::{signal, SigHandler, Signal};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(name = "recchroot")]
#[command(about = "Enter chroot with proper bind mounts (like arch-chroot)")]
struct Args {
    /// Chroot directory (e.g., /mnt)
    chroot_dir: String,

    /// Command to run (default: /bin/bash)
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

// Mounts we need to set up (in order)
const BIND_MOUNTS: &[(&str, &str)] = &[
    ("/proc", "proc"),
    ("/sys", "sys"),
    ("/dev", "dev"),
    ("/run", "run"),
];

// Optional mounts (only if source exists)
const OPTIONAL_MOUNTS: &[&str] = &[
    "/sys/firmware/efi/efivars",
];

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("recchroot: {:#}", e);
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<u8> {
    let args = Args::parse();

    let chroot_dir = Path::new(&args.chroot_dir);
    if !chroot_dir.exists() {
        bail!("Chroot directory {} does not exist", args.chroot_dir);
    }
    if !chroot_dir.is_dir() {
        bail!("{} is not a directory", args.chroot_dir);
    }

    let chroot_canonical = chroot_dir.canonicalize()?;
    let mut mounted: Vec<PathBuf> = Vec::new();

    // Set up signal handler for cleanup
    unsafe {
        let _ = signal(Signal::SIGINT, SigHandler::SigIgn);
    }

    // Setup bind mounts
    for (src, name) in BIND_MOUNTS {
        let target = chroot_canonical.join(name);

        if !target.exists() {
            fs::create_dir_all(&target)
                .with_context(|| format!("Failed to create {}", target.display()))?;
        }

        mount(
            Some(Path::new(src)),
            &target,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )
        .with_context(|| format!("Failed to bind mount {} to {}", src, target.display()))?;

        mounted.push(target);
    }

    // Optional mounts (like efivars)
    for src in OPTIONAL_MOUNTS {
        let src_path = Path::new(src);
        if !src_path.exists() {
            continue;
        }

        let rel_path = src.strip_prefix('/').unwrap_or(src);
        let target = chroot_canonical.join(rel_path);

        if !target.exists() {
            if let Err(_) = fs::create_dir_all(&target) {
                continue; // Skip if we can't create the directory
            }
        }

        if mount(
            Some(src_path),
            &target,
            None::<&str>,
            MsFlags::MS_BIND,
            None::<&str>,
        )
        .is_ok()
        {
            mounted.push(target);
        }
    }

    // Copy resolv.conf for DNS
    let resolv_src = Path::new("/etc/resolv.conf");
    let resolv_dst = chroot_canonical.join("etc/resolv.conf");
    if resolv_src.exists() {
        let _ = fs::copy(resolv_src, &resolv_dst);
    }

    // Determine command to run
    let (cmd, cmd_args): (&str, Vec<&str>) = if args.command.is_empty() {
        ("/bin/bash", vec![])
    } else {
        (
            args.command[0].as_str(),
            args.command[1..].iter().map(|s| s.as_str()).collect(),
        )
    };

    // Run chroot
    let status = Command::new("chroot")
        .arg(&chroot_canonical)
        .arg(cmd)
        .args(&cmd_args)
        .status()
        .context("Failed to run chroot")?;

    // Cleanup: unmount in reverse order
    for target in mounted.iter().rev() {
        let _ = umount2(target, MntFlags::MNT_DETACH);
    }

    Ok(status.code().unwrap_or(1) as u8)
}
