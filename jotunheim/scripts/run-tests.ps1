# Jotunheim: Run all tests in container.
# Usage: from repo root: .\jotunheim\scripts\run-tests.ps1
#        or from jotunheim/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$JotunheimDir = Split-Path -Parent $ScriptDir
Set-Location $JotunheimDir
docker compose -f docker-compose.test.yml run --rm jotunheim-test
