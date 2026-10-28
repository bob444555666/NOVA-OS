#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD="$ROOT/build"
cd "$ROOT"

# Use the official prebuilt Limine release so the ISO build does not depend
# on rebuilding the bootloader from source.
LIMINE_VERSION="12.9.6"
LIMINE_URL="https://github.com/limine-bootloader/limine/releases/download/v${LIMINE_VERSION}/limine-binary.tar.xz"
LIMINE_ARCHIVE="$BUILD/limine-binary.tar.xz"
LIMINE_ROOT="$BUILD/limine-binary"

for tool in cargo rustup xorriso curl tar; do
  command -v "$tool" >/dev/null || { echo "Missing required tool: $tool" >&2; exit 1; }
done

mkdir -p "$BUILD"
if [[ ! -d "$LIMINE_ROOT" ]]; then
  mkdir -p "$LIMINE_ROOT"
  curl --fail --location --retry 3 "$LIMINE_URL" -o "$LIMINE_ARCHIVE"
  tar -xJf "$LIMINE_ARCHIVE" -C "$LIMINE_ROOT" --strip-components=1
fi

LIMINE_TOOL="$(find "$LIMINE_ROOT" -type f -name limine -print -quit)"
[[ -n "$LIMINE_TOOL" ]] || { echo "Could not find the Limine host tool in the downloaded archive." >&2; exit 1; }
LIMINE_DIR="$(dirname "$LIMINE_TOOL")"

for file in limine limine-bios.sys limine-bios-cd.bin limine-uefi-cd.bin BOOTX64.EFI; do
  [[ -f "$LIMINE_DIR/$file" ]] || { echo "Missing Limine boot file: $file" >&2; exit 1; }
done

rustup target add x86_64-unknown-none
cargo build --release --target x86_64-unknown-none

STAGING="$BUILD/iso-root"
rm -rf "$STAGING"
mkdir -p "$STAGING/boot" "$STAGING/EFI/BOOT"
cp "$ROOT/limine.conf" "$STAGING/limine.conf"
cp "$ROOT/target/x86_64-unknown-none/release/nova-kernel" "$STAGING/boot/nova-kernel"
cp "$LIMINE_DIR/limine-bios.sys" "$STAGING/limine-bios.sys"
cp "$LIMINE_DIR/limine-bios-cd.bin" "$STAGING/limine-bios-cd.bin"
cp "$LIMINE_DIR/limine-uefi-cd.bin" "$STAGING/limine-uefi-cd.bin"
cp "$LIMINE_DIR/BOOTX64.EFI" "$STAGING/EFI/BOOT/BOOTX64.EFI"

xorriso -as mkisofs \
  -R -r -J \
  -b limine-bios-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table \
  --efi-boot limine-uefi-cd.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  "$STAGING" -o "$BUILD/nova-os.iso"

"$LIMINE_TOOL" bios-install "$BUILD/nova-os.iso"
test -s "$BUILD/nova-os.iso"
echo "Built successfully: $BUILD/nova-os.iso"
