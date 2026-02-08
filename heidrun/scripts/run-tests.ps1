# Heidrun: Run all tests in container.
# Usage: from repo root: .\heidrun\scripts\run-tests.ps1
#        or from heidrun/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$HeidrunDir = Split-Path -Parent $ScriptDir
Set-Location $HeidrunDir
docker compose -f docker-compose.test.yml run --rm heidrun-test
