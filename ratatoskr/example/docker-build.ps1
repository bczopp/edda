# Docker Build Script for Windows PowerShell
# Builds the Ratatoskr example project in a Docker container

docker build -t ratatoskr-example:latest --target builder -f Dockerfile .

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
} else {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}
