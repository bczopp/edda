# Læraðr: Run all tests in container.
# Usage: from repo root: .\laeradr\scripts\run-tests.ps1
#        or from laeradr/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$LaeradrDir = Split-Path -Parent $ScriptDir
Set-Location $LaeradrDir
docker compose -f docker-compose.test.yml run --rm laeradr-test
