# Huginn-Muninn: Run all tests in container (Phase 1; see docker-compose.test.yml for dependencies).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\huginn-muninn\scripts\run-tests.ps1
#        or from huginn-muninn/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm huginn-muninn-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$HMDir = Split-Path -Parent $ScriptDir
Set-Location $HMDir
docker compose -f docker-compose.test.yml run --rm huginn-muninn-test
