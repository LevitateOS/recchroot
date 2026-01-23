# recchroot

LevitateOS chroot helper. Like `arch-chroot` for Arch Linux - sets up bind mounts and enters chroot properly.

## Status

| Metric | Value |
|--------|-------|
| Stage | Beta |
| Target | x86_64 Linux |
| Last verified | 2026-01-23 |

### Works

- Bind mounts for /proc, /sys, /dev, /run, efivars
- DNS resolution via resolv.conf copy
- Automatic cleanup on exit
- Protected path blocking

### Known Issues

- See parent repo issues

---

## Author

<!-- HUMAN WRITTEN - DO NOT MODIFY -->

[Waiting for human input]

<!-- END HUMAN WRITTEN -->

---

**You run commands inside the chroot yourself.** This tool enters chroot, nothing more.

## Usage

```bash
# After extracting the system with recstrap
recstrap /mnt

# Enter the chroot
recchroot /mnt

# Now you're inside the chroot - do your configuration
passwd
bootctl install
exit
```

## Options

```
USAGE:
    recchroot <CHROOT_DIR> [COMMAND]...

ARGS:
    <CHROOT_DIR>    Chroot directory (e.g., /mnt)
    [COMMAND]...    Command to run (default: /bin/bash)

OPTIONS:
    -h, --help    Print help
```

## Examples

```bash
# Interactive shell in chroot
recchroot /mnt

# Run a single command
recchroot /mnt passwd

# Run bootloader installation
recchroot /mnt bootctl install

# Run multiple commands
recchroot /mnt bash -c "passwd && bootctl install"
```

## What recchroot does

- Bind mounts `/proc`, `/sys`, `/dev`, `/run` into the chroot
- Bind mounts `/sys/firmware/efi/efivars` if it exists (for UEFI)
- Copies `/etc/resolv.conf` for DNS resolution
- Enters the chroot and runs your command (or interactive shell)
- Cleans up all bind mounts on exit

## What recchroot does NOT do

- Run any installation commands automatically
- Configure the system for you
- Install bootloaders, set passwords, or create users
- Any other installation step

This is intentional. LevitateOS is for users who want control, like Arch.

## Requirements

- Linux only (uses Linux-specific mount syscalls)
- Root privileges required
- Target directory must contain a valid Linux filesystem
- Target cannot be a protected system path (/, /usr, /etc, etc.)

## Error Codes

| Code | Exit | Description |
|------|------|-------------|
| E001 | 1 | Target directory does not exist |
| E002 | 2 | Target is not a directory |
| E003 | 3 | Failed to create mount point directory |
| E004 | 4 | Mount operation failed |
| E005 | 5 | Unmount operation failed (warning only) |
| E006 | 6 | Command execution failed |
| E007 | 7 | Must run as root |
| E008 | 8 | Target is a protected system path |

## Protected Paths

These paths cannot be used as chroot targets:

`/`, `/bin`, `/boot`, `/dev`, `/etc`, `/home`, `/lib`, `/lib64`, `/opt`, `/proc`, `/root`, `/run`, `/sbin`, `/srv`, `/sys`, `/tmp`, `/usr`, `/var`

## Building

```bash
cargo build --release
```

## License

MIT
