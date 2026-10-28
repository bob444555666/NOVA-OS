# NOVA Independent Kernel: engineering plan

This directory documents the independent-kernel track. The Linux-based NOVA Desktop Edition is a separate way to provide a useful desktop while this kernel grows.

## Current verified scope

- Limine boots the x86-64 Rust kernel.
- The kernel writes diagnostics to the COM1 serial port.
- It can draw a basic framebuffer preview when Limine supplies a compatible framebuffer.
- The kernel installs its own 256-entry x86-64 IDT with a fail-stop handler. Hardware interrupts intentionally remain disabled until PIC/APIC routing and interrupt-safe handlers are implemented.
- The kernel includes an allocation-free interactive serial monitor with `help`, `about`, `version`, `clear`, and `halt` commands.
- Limine's real memory-map response is now requested and parsed at boot. Only page-aligned `USABLE` entries are released to the frame allocator; all other entry types remain reserved.
- The boot path logs memory-map and allocator counts, then performs a one-frame allocate/free smoke check. The bitmap currently manages physical addresses below 32 GiB; higher memory is explicitly reported as unmanaged.
- The physical frame allocator has host-side tests for usable/reserved frames, freeing/reuse, invalid frees, and bitmap capacity. Run them with `bash scripts/test-memory.sh`.

This is an early kernel monitor, **not** a full operating system. The serial monitor is for development and is not a substitute for keyboard/mouse support or a graphical shell.

## Build toward a usable independent OS

The kernel must be advanced in small, testable stages. Each stage should be covered by automated QEMU tests before hardware claims are made.

1. **Boot and diagnostics** — boot protocol validation, serial logging, panic handling, QEMU BIOS and UEFI smoke tests.
2. **CPU foundations** — [initial step implemented] install a kernel-owned IDT and fail-stop handler; next: per-vector exception diagnostics, APIC/PIC setup, timer interrupts, CPU feature detection.
3. **Memory management** — [initial step implemented] parse Limine memory map and initialize physical page allocator; next: virtual address-space management, kernel heap, guard pages.
4. **Input and display** — PS/2 keyboard as a first compatibility path, USB xHCI/HID later, framebuffer console, cursor, event queue.
5. **Processes and syscalls** — privilege separation, context switching, scheduler, user-mode execution, syscall ABI, ELF loader.
6. **Storage and filesystems** — block-device abstraction, AHCI/NVMe drivers, read-only FAT support for boot media, then a journaled writable filesystem.
7. **Userspace** — init process, shell, process utilities, permissions, configuration and logging services.
8. **Networking** — PCI enumeration, NIC drivers, Ethernet, ARP, IPv4/IPv6, DHCP, DNS, TCP/UDP, TLS validation and socket API.
9. **Window system** — input routing, compositing, movable windows, fonts, themes, accessibility and desktop services.
10. **Applications and security** — documented app ABI/SDK, package format, signed updates, sandboxing, permissions, recovery and installer.

## Non-negotiable engineering rules

- Never claim hardware support without testing on the named hardware.
- Never install to or repartition a user's internal disk as part of automated tests.
- Keep the Linux-based desktop build separate from the independent kernel so neither is misrepresented.
- Require QEMU tests for boot, exceptions, allocator invariants, input events, filesystem operations, networking and userspace before declaring those features working.
- Keep bootable release artifacts attached to successful builds only.

## Near-term next milestone

The allocator is now initialized from Limine's real memory map and used for a boot-time allocate/free smoke check. It only manages the first 32 GiB of physical address space; a larger or sparse-memory-aware allocator is still needed for systems whose usable RAM lies beyond that cap. The next milestone is a tested IDT and exception/interrupt layer. The kernel still has no working PS/2/USB input, storage/filesystem, process scheduler, user-mode programs, networking, or interactive graphical desktop.
