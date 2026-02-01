# Nornen: Run all tests in container (Phase 1; see docker-compose.test.yml for dependencies).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\nornen\scripts\run-tests.ps1
#        or from nornen/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm nornen-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NornenDir = Split-Path -Parent $ScriptDir
Set-Location $NornenDir
docker compose -f docker-compose.test.yml run --rm nornen-test
