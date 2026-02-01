# Midgard: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\midgard\scripts\run-tests.ps1
#        or from midgard/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$MidgardDir = Split-Path -Parent $ScriptDir
Set-Location $MidgardDir
docker compose -f docker-compose.test.yml run --rm midgard-test
