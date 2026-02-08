# Geri: Run all tests in container (Phase 1; depends on postgres, mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\geri\scripts\run-tests.ps1
#        or from geri/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm geri-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$GeriDir = Split-Path -Parent $ScriptDir
Set-Location $GeriDir
docker compose -f docker-compose.test.yml run --rm geri-test
