# Thor: Run all tests in container (Phase 1 / Phase 15).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\thor\scripts\run-tests.ps1
#        or from thor/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm thor-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ThorDir = Split-Path -Parent $ScriptDir
Set-Location $ThorDir
docker compose -f docker-compose.test.yml run --rm thor-test
