# NOVA OS

NOVA OS is an experimental open-source operating-system project for 64-bit Intel/AMD PCs. The first milestone is a small kernel that boots from a UEFI-compatible USB drive or virtual machine; this is not yet a daily-use operating system.

## Current milestone
- Rust `no_std` kernel targeting `x86_64-unknown-none`
- Limine boot protocol and bootloader
- Framebuffer discovery and a simple graphical boot screen when available
- ISO build script and CI build check

## Build (Linux)
Install Rust via rustup, plus `make`, `xorriso`, `curl`, and `tar`. Then run:

```sh
bash scripts/build-iso.sh
```

The script downloads a pinned Limine release into `build/` and creates `build/nova-os.iso`. A network connection is needed on the first build.

## Test safely
1. Start with QEMU or another virtual machine and test the ISO there.
2. For USB testing, write the ISO to a spare USB drive using a trusted imaging utility. **Writing an image erases the selected drive.**
3. Boot using your firmware's one-time boot menu. Installation to an internal drive is not implemented.

UEFI x86-64 is the initial target. Keyboard, storage, Wi-Fi, sound, power management, and a full desktop are not implemented yet.

## Roadmap
1. Bootable kernel and graphical status screen
2. CPU exceptions, memory management, timer and keyboard input
3. Text console, shell, and basic desktop compositor
4. Filesystem and basic applications
5. Hardware drivers, testing, and release automation
6. Optional online release/update metadata services

## License
MIT; see [LICENSE](LICENSE).
