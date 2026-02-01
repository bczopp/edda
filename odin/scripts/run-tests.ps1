# Odin: Run all tests in container (Phase 1; uses run-tests-with-mocks.sh inside container).
# Usage: from repo root: .\odin\scripts\run-tests.ps1
#        or from odin/: .\scripts\run-tests.ps1
# Single test: docker compose -f docker-compose.test.yml run --rm odin-test cargo test <test_name>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$OdinDir = Split-Path -Parent $ScriptDir
Set-Location $OdinDir
docker compose -f docker-compose.test.yml run --rm odin-test
