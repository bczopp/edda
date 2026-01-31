# Docker Test Script for Windows PowerShell
# Runs tests for the Ratatoskr example project in a Docker container

docker-compose -f docker-compose.test.yml run --rm test

if ($LASTEXITCODE -eq 0) {
    Write-Host "Tests passed!" -ForegroundColor Green
} else {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}
