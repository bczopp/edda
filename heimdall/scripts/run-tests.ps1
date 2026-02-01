# Heimdall: Run all tests in container (Phase 1; depends on Postgres).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\heimdall\scripts\run-tests.ps1
#        or from heimdall/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm heimdall-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$HeimdallDir = Split-Path -Parent $ScriptDir
Set-Location $HeimdallDir
docker compose -f docker-compose.test.yml run --rm heimdall-test
