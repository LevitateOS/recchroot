# recchroot

LevitateOS chroot helper. Like `arch-chroot` for Arch Linux - sets up bind mounts and enters chroot properly.

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

## Building

```bash
cargo build --release
```

## License

MIT
