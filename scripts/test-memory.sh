#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$(mktemp "${TMPDIR:-/tmp}/nova-memory-tests.XXXXXX")"
trap 'rm -f "$BIN"' EXIT

rustc --edition=2021 --test "$ROOT/tests/frame_allocator.rs" -o "$BIN"
"$BIN"
