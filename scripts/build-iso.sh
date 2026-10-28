#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD="$ROOT/build"
cd "$ROOT"
LIMINE_VERSION="8.6.0"
LIMINE_URL="https://github.com/limine-bootloader/limine/releases/download/v${LIMINE_VERSION}/limine-${LIMINE_VERSION}.tar.xz"

command -v cargo >/dev/null || { echo "Missing cargo (install Rust via rustup)." >&2; exit 1; }
command -v xorriso >/dev/null || { echo "Missing xorriso. Install it with your Linux package manager." >&2; exit 1; }
command -v curl >/dev/null || { echo "Missing curl." >&2; exit 1; }
command -v tar >/dev/null || { echo "Missing tar." >&2; exit 1; }

mkdir -p "$BUILD"
if [[ ! -d "$BUILD/limine-${LIMINE_VERSION}" ]]; then
  curl --fail --location --retry 3 "$LIMINE_URL" -o "$BUILD/limine.tar.xz"
  tar -xJf "$BUILD/limine.tar.xz" -C "$BUILD"
fi

LIMINE_DIR="$BUILD/limine-${LIMINE_VERSION}"
if [[ ! -f "$LIMINE_DIR/limine" ]]; then
  make -C "$LIMINE_DIR"
fi

rustup target add x86_64-unknown-none
cargo build --manifest-path "$ROOT/Cargo.toml" --release --target x86_64-unknown-none

STAGING="$BUILD/iso-root"
rm -rf "$STAGING"
mkdir -p "$STAGING/boot"
cp "$ROOT/limine.conf" "$STAGING/limine.conf"
cp "$ROOT/target/x86_64-unknown-none/release/nova-kernel" "$STAGING/boot/nova-kernel"
cp "$LIMINE_DIR/limine-bios.sys" "$STAGING/limine-bios.sys"
cp "$LIMINE_DIR/limine-bios-cd.bin" "$STAGING/limine-bios-cd.bin"
cp "$LIMINE_DIR/limine-uefi-cd.bin" "$STAGING/limine-uefi-cd.bin"

xorriso -as mkisofs \
  -R -r -J \
  -b limine-bios-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table \
  --efi-boot limine-uefi-cd.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  "$STAGING" -o "$BUILD/nova-os.iso"

"$LIMINE_DIR/limine" bios-install "$BUILD/nova-os.iso"
echo "Built: $BUILD/nova-os.iso"
