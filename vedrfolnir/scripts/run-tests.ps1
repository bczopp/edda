# Vedrfolnir: Run all tests in container.
# Usage: from repo root: .\vedrfolnir\scripts\run-tests.ps1
#        or from vedrfolnir/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$VedrfolnirDir = Split-Path -Parent $ScriptDir
Set-Location $VedrfolnirDir
docker compose -f docker-compose.test.yml run --rm vedrfolnir-test
