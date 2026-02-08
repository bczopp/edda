# Jotunheim: Build release and report binary size (Phase 10.2.1).
# Usage: from jotunheim/: .\scripts\check-size.ps1
# Optional: $env:MAX_BYTES = 5000000; .\scripts\check-size.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$JotunheimDir = Split-Path -Parent $ScriptDir
Set-Location $JotunheimDir

cargo build --release -p jotunheim-esp32
$bin = Join-Path $JotunheimDir "target\release\jotunheim_esp32.exe"
if (-not (Test-Path $bin)) { $bin = Join-Path $JotunheimDir "target\release\jotunheim_esp32" }
if (-not (Test-Path $bin)) { Write-Error "Binary not found"; exit 1 }
$size = (Get-Item $bin).Length
Write-Host "Binary size: $size bytes"
if ($env:MAX_BYTES -and [int]$size -gt [int]$env:MAX_BYTES) {
  Write-Error "Exceeds limit $env:MAX_BYTES bytes"
  exit 1
}
