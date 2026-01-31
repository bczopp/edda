#!/bin/bash
# Docker Test Script for Linux/Mac
# Runs tests for the Ratatoskr example project in a Docker container

docker-compose -f docker-compose.test.yml run --rm test

if [ $? -eq 0 ]; then
    echo "Tests passed!"
else
    echo "Tests failed!"
    exit 1
fi
