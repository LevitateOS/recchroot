# CLAUDE.md - Recchroot

## STOP. READ. THEN ACT.

Before modifying this crate, read `src/main.rs` to understand the chroot setup logic.

---

## What is recchroot?

LevitateOS chroot helper. **Like arch-chroot, NOT like an installer.**

Sets up bind mounts (/dev, /proc, /sys, /run), enters chroot, cleans up on exit. That's it.
User runs commands inside chroot manually.

## Development

```bash
cargo build --release    # LTO + strip enabled
cargo clippy
```

## Key Rules

1. **recchroot = arch-chroot** - Just enter chroot properly, nothing else
2. **Keep it simple** - ~100 lines, one job
3. **No automation** - User runs commands manually inside chroot
4. **Always cleanup** - Unmount bind mounts even on error/signal

## What recchroot does

```bash
recchroot /mnt              # Interactive shell in chroot
recchroot /mnt passwd       # Run single command in chroot
recchroot /mnt bootctl install  # Run bootloader install in chroot
```

## What recchroot does NOT do

- Run installation commands automatically
- Configure the system (user does that)
- Install bootloader (user does that)
- Any other installation step

## Mounts set up

1. `/proc` -> `<chroot>/proc`
2. `/sys` -> `<chroot>/sys`
3. `/dev` -> `<chroot>/dev`
4. `/run` -> `<chroot>/run`
5. `/sys/firmware/efi/efivars` -> `<chroot>/sys/firmware/efi/efivars` (if exists)
6. Copies `/etc/resolv.conf` for DNS resolution

## Testing

Test in QEMU with a mounted target directory, verify mounts are created and cleaned up.
