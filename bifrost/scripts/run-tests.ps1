# Bifrost: Run all tests in container (Phase 1.2.1 / Phase 20).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\bifrost\scripts\run-tests.ps1
#        or from bifrost/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm bifrost-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$BifrostDir = Split-Path -Parent $ScriptDir
Set-Location $BifrostDir
docker compose -f docker-compose.test.yml run --rm bifrost-test
