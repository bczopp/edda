# Yggdrasil: Run all tests in container (Elixir).
# Runs: mix test (see docker-compose.test.yml).
# Usage: from repo root: .\yggdrasil\scripts\run-tests.ps1
#        or from yggdrasil/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$YggdrasilDir = Split-Path -Parent $ScriptDir
Set-Location $YggdrasilDir
docker compose -f docker-compose.test.yml run --rm yggdrasil-test
