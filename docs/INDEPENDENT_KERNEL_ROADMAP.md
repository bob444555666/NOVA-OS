# NOVA Independent Kernel: engineering plan

This directory documents the independent-kernel track. The Linux-based NOVA Desktop Edition is a separate way to provide a useful desktop while this kernel grows.

## Current verified scope

- Limine boots the x86-64 Rust kernel.
- The kernel writes diagnostics to the COM1 serial port.
- It can draw a basic framebuffer preview when Limine supplies a compatible framebuffer.
- The kernel now includes an allocation-free interactive serial monitor with `help`, `about`, `version`, `clear`, and `halt` commands.

This is an early kernel monitor, **not** a full operating system. The serial monitor is for development and is not a substitute for keyboard/mouse support or a graphical shell.

## Build toward a usable independent OS

The kernel must be advanced in small, testable stages. Each stage should be covered by automated QEMU tests before hardware claims are made.

1. **Boot and diagnostics** — boot protocol validation, serial logging, panic handling, QEMU BIOS and UEFI smoke tests.
2. **CPU foundations** — GDT, IDT, exception handlers, APIC/PIC setup, timer interrupts, CPU feature detection.
3. **Memory management** — parse Limine memory map, physical page allocator, virtual address-space management, kernel heap, guard pages.
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

The next milestone is a testable interrupt/exception layer and memory-map-driven physical page allocator. Those are prerequisites for safe multitasking, user programs, storage, networking and a real graphical desktop.
