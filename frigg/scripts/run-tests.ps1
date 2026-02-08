# Frigg: Run all tests in container.
# Usage: from repo root: .\frigg\scripts\run-tests.ps1
#        or from frigg/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$FriggDir = Split-Path -Parent $ScriptDir
Set-Location $FriggDir
docker compose -f docker-compose.test.yml run --rm frigg-test
