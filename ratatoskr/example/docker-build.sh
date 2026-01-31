#!/bin/bash
# Docker Build Script for Linux/Mac
# Builds the Ratatoskr example project in a Docker container

docker build -t ratatoskr-example:latest --target builder -f Dockerfile .

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi
