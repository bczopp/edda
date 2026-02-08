# Jotunheim: Flash ESP32 firmware (requires esp-rs toolchain and connected device).
# Usage: from repo root: .\jotunheim\scripts\flash.ps1
#        or from jotunheim/: .\scripts\flash.ps1
#
# Prerequisites: espup, espflash; build for ESP32 first (scripts/build.ps1 --esp32).

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$JotunheimDir = Split-Path -Parent $ScriptDir
Set-Location $JotunheimDir

# Build for ESP32 if needed, then flash
cargo espflash flash --release -p jotunheim-esp32 --target xtensa-esp32-espidf --monitor
