# Nidhöggr: Run all tests in container (build context: repo root).
# Usage: from repo root: .\nidhoggr\scripts\run-tests.ps1
#        or from nidhoggr/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$NidhoggrDir = Split-Path -Parent $ScriptDir
Set-Location $NidhoggrDir
docker compose -f docker-compose.test.yml run --rm nidhoggr-test
