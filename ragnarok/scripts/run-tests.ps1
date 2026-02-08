# Ragnarok: Run all tests in container.
# Usage: from repo root: .\ragnarok\scripts\run-tests.ps1
#        or from ragnarok/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RagnarokDir = Split-Path -Parent $ScriptDir
Set-Location $RagnarokDir
docker compose -f docker-compose.test.yml run --rm ragnarok-test
