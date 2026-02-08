# Hirtir: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\hirtir\scripts\run-tests.ps1
#        or from hirtir/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$HirtirDir = Split-Path -Parent $ScriptDir
Set-Location $HirtirDir
docker compose -f docker-compose.test.yml run --rm hirtir-test
