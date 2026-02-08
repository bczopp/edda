# Ratatoskr: Run all tests in container.
# Usage: from repo root: .\ratatoskr\scripts\run-tests.ps1
#        or from ratatoskr/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RatatoskrDir = Split-Path -Parent $ScriptDir
Set-Location $RatatoskrDir
docker compose -f docker-compose.test.yml run --rm ratatoskr-test
