# NOVA OS

NOVA OS is an experimental open-source operating-system project for 64-bit Intel/AMD PCs. The first milestone is a small kernel that boots from a UEFI-compatible USB drive or virtual machine; this is not yet a daily-use operating system.

## Current milestone

- Rust `no_std` kernel targeting `x86_64-unknown-none`
- Limine boot protocol and bootloader
- Framebuffer discovery and a simple graphical boot screen when available
- Serial boot diagnostics and a development-only serial monitor
- A kernel-owned IDT with a fail-stop handler (hardware IRQs remain disabled)
- Limine memory-map parsing and an initial physical frame allocator
- ISO build script and automated QEMU BIOS/UEFI boot checks

## Build (Linux)

Install Rust via rustup, plus `make`, `xorriso`, `curl`, and `tar`. Then run:

```sh
bash scripts/build-iso.sh
```

The script downloads a pinned Limine binary release into `build/` and creates `build/nova-os.iso`. A network connection is needed on the first build. GitHub Actions runs the same ISO build, boots it in QEMU, and uploads an artifact named `nova-os-iso` if the checks pass.

## Test safely

1. Open [GitHub Actions](https://github.com/bob444555666/NOVA-OS/actions) and download `nova-os-iso` from a successful run.
2. Start with QEMU or another virtual machine and test the ISO there.
3. For USB testing, write the ISO to a spare USB drive using a trusted imaging utility. **Writing an image erases the selected drive.**
4. Boot using your firmware's one-time boot menu. Installation to an internal drive is not implemented.

UEFI x86-64 is the initial target. This is **not yet a daily-use or complete independent OS**. Working keyboard/mouse drivers, virtual memory, scheduling, user-mode processes, storage/filesystems, networking, sound, power management, a window manager and application runtime remain unfinished. The framebuffer is currently a static boot preview, not an interactive desktop. See [`docs/INDEPENDENT_KERNEL_ROADMAP.md`](docs/INDEPENDENT_KERNEL_ROADMAP.md) for the staged engineering plan.

## Roadmap

1. Bootable kernel and graphical status screen
2. CPU exceptions, memory management, timer and keyboard input
3. Text console, shell, and basic desktop compositor
4. Filesystem and basic applications
5. Hardware drivers, testing, and release automation
6. Optional online release/update metadata services

## License

MIT; see [LICENSE](LICENSE).
