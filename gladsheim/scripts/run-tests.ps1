# Gladsheim: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: .\gladsheim\scripts\run-tests.ps1
#        or from gladsheim/: .\scripts\run-tests.ps1

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$GladsheimDir = Split-Path -Parent $ScriptDir
Set-Location $GladsheimDir
docker compose -f docker-compose.test.yml run --rm gladsheim-test
