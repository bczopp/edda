# Freki: Run all tests in container (Phase 1; depends on qdrant, redis, mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\freki\scripts\run-tests.ps1
#        or from freki/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm freki-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$FrekiDir = Split-Path -Parent $ScriptDir
Set-Location $FrekiDir
docker compose -f docker-compose.test.yml run --rm freki-test
