# NOVA OS Desktop Edition

NOVA Desktop is the practical, bootable desktop edition of NOVA OS. It uses Debian's Linux kernel and hardware ecosystem under a NOVA-branded desktop, rather than claiming the small experimental Rust kernel already implements every driver and application subsystem.

The goal is a cohesive Windows/macOS-inspired experience with Linux flexibility: a desktop and window manager, application menu, taskbar/panel, file manager, settings, network and audio tools, and a set of real applications.

## Included applications and system features

- XFCE desktop, window manager, panel, app menu, notifications, power settings, and file manager
- Firefox ESR and Chromium web browsers
- LibreOffice Writer, Calc, and Impress for documents, spreadsheets, and presentations
- VLC media player and GIMP image editor
- Archive manager and PDF viewer
- NetworkManager, Wi-Fi UI, Bluetooth tools, PipeWire audio, and volume controls
- Printer support and disk utilities
- Common Wi-Fi firmware packages for supported hardware
- Git, terminal, Flatpak support, and NOVA-branded wallpaper/desktop shortcuts

Hardware compatibility depends on the machine and available firmware; no Linux distribution can promise every Wi-Fi adapter, graphics chip, fingerprint reader, or sleep/power feature works on every laptop without testing.

## Build the ISO

On Debian or Ubuntu with root/sudo access, install the tools and build from this directory:

```sh
sudo apt-get update
sudo apt-get install --yes live-build debootstrap squashfs-tools xorriso grub-pc-bin grub-efi-amd64-bin mtools dosfstools
cd nova-live
sudo lb config \
  --distribution trixie \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --debian-installer live \
  --linux-packages "linux-image-amd64" \
  --archive-areas "main contrib non-free non-free-firmware" \
  --mirror-bootstrap "http://deb.debian.org/debian/" \
  --mirror-chroot "http://deb.debian.org/debian/" \
  --mirror-binary "http://deb.debian.org/debian/" \
  --mirror-chroot-security "http://security.debian.org/debian-security/" \
  --mirror-binary-security "http://security.debian.org/debian-security/" \
  --iso-application "NOVA OS Desktop" \
  --iso-publisher "NOVA OS Project" \
  --iso-volume "NOVA_DESKTOP"
sudo lb build
```

The expected output is `nova-live/live-image-amd64.hybrid.iso`. GitHub Actions is configured to build the ISO and upload it as the `nova-desktop-iso` artifact when the build succeeds.

## Safe testing

1. Test the ISO in a virtual machine first.
2. For USB testing, use a tool such as Rufus or balenaEtcher to write the ISO to a USB drive. Writing it erases that USB drive.
3. Boot from the USB and choose the live desktop to test hardware before installing anything.
4. Do not repartition or install to the internal Windows drive unless you deliberately choose to do so and have backed up important files.

## Honest project status

This is a Debian-based NOVA Desktop distribution, not yet a wholly independent kernel/OS implementation. The bare-metal Rust kernel remains a separate experimental project. A truly independent kernel with its own drivers, filesystem, networking stack, security model, graphics stack, and application ecosystem is a much larger ongoing engineering project.
