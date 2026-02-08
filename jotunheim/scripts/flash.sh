#!/usr/bin/env bash
# Jotunheim: Flash ESP32 firmware (requires esp-rs toolchain and connected device).
# Usage: from repo root: jotunheim/scripts/flash.sh
#        or from jotunheim/: ./scripts/flash.sh
#
# Prerequisites: espup, espflash; build for ESP32 first (scripts/build.sh --esp32).

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JOTUNHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$JOTUNHEIM_DIR"

cargo espflash flash --release -p jotunheim-esp32 --target xtensa-esp32-espidf --monitor
