# Loki: Run all tests in container (Phase 1; depends on mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\loki\scripts\run-tests.ps1
#        or from loki/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm loki-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$LokiDir = Split-Path -Parent $ScriptDir
Set-Location $LokiDir
docker compose -f docker-compose.test.yml run --rm loki-test
