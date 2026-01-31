#!/bin/bash
# Docker Development Script for Linux/Mac
# Starts an interactive development container

docker-compose -f docker-compose.test.yml run --rm dev
