# Valkyries: Run all tests in container.
# Usage: from repo root: .\valkyries\scripts\run-tests.ps1
#        or from valkyries/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ValkyriesDir = Split-Path -Parent $ScriptDir
Set-Location $ValkyriesDir
docker compose -f docker-compose.test.yml run --rm valkyries-test
