# Jotunheim: Build workspace (host) or prepare for ESP32.
# Usage: from repo root: .\jotunheim\scripts\build.ps1 [--esp32]
#        or from jotunheim/: .\scripts\build.ps1 [--esp32]
#
# Without --esp32: cargo build (for host, e.g. tests).
# With --esp32: requires ESP32 toolchain (esp-rs); builds for xtensa-esp32-espidf.
#   Install: cargo install espup && espup install
#   Then: cargo build --release -p jotunheim-esp32 --target xtensa-esp32-espidf

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$JotunheimDir = Split-Path -Parent $ScriptDir
Set-Location $JotunheimDir

if ($args -contains "--esp32") {
    Write-Host "ESP32 build: ensure esp-rs toolchain is installed (espup)."
    cargo build --release -p jotunheim-esp32 --target xtensa-esp32-espidf
} else {
    cargo build --release
}
