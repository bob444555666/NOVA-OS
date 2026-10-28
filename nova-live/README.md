# NOVA OS Desktop Edition

This is the **usable desktop edition** build path. It is separate from the experimental Rust kernel prototype at the repository root, because the current bare-metal kernel does not yet have input drivers, a filesystem, networking, or a browser runtime.

NOVA Desktop is built as a Debian Live system with the XFCE desktop, a window manager, file manager, settings tools, network manager, audio tools, and browsers. This gives the project a real operating-system foundation while we develop NOVA branding and custom components.

## Build

On Debian or Ubuntu with root/sudo access:

```sh
sudo apt-get update
sudo apt-get install --yes live-build
cd nova-live
sudo lb config \
  --distribution trixie \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --debian-installer live \
  --archive-areas "main contrib non-free non-free-firmware" \
  --bootloaders "grub-efi grub-pc" \
  --iso-application "NOVA OS Desktop" \
  --iso-publisher "NOVA OS Project" \
  --iso-volume "NOVA_DESKTOP"
sudo lb build
```

The ISO is produced in `nova-live/live-image-amd64.hybrid.iso`.

## Included working desktop

- XFCE desktop with movable/resizable windows, panel, application menu and window manager
- File manager, terminal, settings manager and screenshot utility
- Network manager and Wi-Fi UI
- Audio controls and PipeWire
- Chromium and Firefox ESR browsers
- Disk tools, Git and common desktop utilities
- NOVA-branded wallpaper and desktop shortcuts

## Important

This edition uses Debian's Linux kernel and hardware support. It is not the same as the experimental Rust kernel. It is the practical route to a desktop that behaves like a real OS on the HP laptop. Test the ISO in a virtual machine first; writing an ISO to USB erases the selected USB drive. Do not install to or repartition the internal Windows drive unless you intentionally choose to do so.
