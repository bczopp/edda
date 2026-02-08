#!/usr/bin/env bash
# Jotunheim: Build workspace (host) or prepare for ESP32.
# Usage: from repo root: jotunheim/scripts/build.sh [--esp32]
#        or from jotunheim/: ./scripts/build.sh [--esp32]
#
# Without --esp32: cargo build (for host, e.g. tests).
# With --esp32: requires ESP32 toolchain (esp-rs); builds for xtensa-esp32-espidf.
#   Install: cargo install espup && espup install
#   Then: cargo build --release -p jotunheim-esp32 --target xtensa-esp32-espidf

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JOTUNHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$JOTUNHEIM_DIR"

if [ "${1:-}" = "--esp32" ]; then
  echo "ESP32 build: ensure esp-rs toolchain is installed (espup)."
  cargo build --release -p jotunheim-esp32 --target xtensa-esp32-espidf
else
  cargo build --release
fi
