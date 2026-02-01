# Eikthyrnir: Run all tests in container.
# Usage: from repo root: .\eikthyrnir\scripts\run-tests.ps1
#        or from eikthyrnir/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$EikthyrnirDir = Split-Path -Parent $ScriptDir
Set-Location $EikthyrnirDir
docker compose -f docker-compose.test.yml run --rm eikthyrnir-test
