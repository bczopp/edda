# Alfheim: Run all tests in container (Bun).
# Runs: bun test (see docker-compose.test.yml).
# Usage: from repo root: .\alfheim\scripts\run-tests.ps1
#        or from alfheim/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$AlfheimDir = Split-Path -Parent $ScriptDir
Set-Location $AlfheimDir
docker compose -f docker-compose.test.yml run --rm alfheim-test
