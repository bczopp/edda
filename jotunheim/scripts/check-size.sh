#!/usr/bin/env bash
# Jotunheim: Build release and report binary size (Phase 10.2.1).
# Usage: from jotunheim/: ./scripts/check-size.sh
# Optional: MAX_BYTES=5000000 ./scripts/check-size.sh  (fail if binary larger)

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JOTUNHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$JOTUNHEIM_DIR"

cargo build --release -p jotunheim-esp32
BIN="target/release/jotunheim_esp32"
if [ -f "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi
if [ ! -f "$BIN" ]; then
  echo "Binary not found: $BIN"
  exit 1
fi
SIZE=$(wc -c < "$BIN" | tr -d ' \n')
echo "Binary size: $SIZE bytes"
if [ -n "${MAX_BYTES:-}" ] && [ "$SIZE" -gt "$MAX_BYTES" ]; then
  echo "Exceeds limit $MAX_BYTES bytes"
  exit 1
fi
