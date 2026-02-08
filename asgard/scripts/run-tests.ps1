# Asgard: Run all tests in container.
# Usage: from repo root: .\asgard\scripts\run-tests.ps1
#        or from asgard/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$AsgardDir = Split-Path -Parent $ScriptDir
Set-Location $AsgardDir
docker compose -f docker-compose.test.yml run --rm asgard-test
