# Forseti: Run all tests in container.
# Usage: from repo root: .\forseti\scripts\run-tests.ps1
#        or from forseti/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ForsetiDir = Split-Path -Parent $ScriptDir
Set-Location $ForsetiDir
docker compose -f docker-compose.test.yml run --rm forseti-test
