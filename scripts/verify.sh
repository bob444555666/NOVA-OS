#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== NOVA: host-side physical frame allocator tests =="
bash scripts/test-memory.sh

echo
echo "== NOVA: build bootable ISO =="
bash scripts/build-iso.sh

echo
echo "Verification steps completed. QEMU BIOS/UEFI boot checks are run by GitHub Actions."
