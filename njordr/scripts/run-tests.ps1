# Njörðr: Run all tests in container.
# Usage: from repo root: .\njordr\scripts\run-tests.ps1
#        or from njordr/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NjordrDir = Split-Path -Parent $ScriptDir
Set-Location $NjordrDir
docker compose -f docker-compose.test.yml run --rm njordr-test
