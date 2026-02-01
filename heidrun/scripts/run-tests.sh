#!/usr/bin/env bash
# Heidrun: Run all tests in container.
# Usage: from repo root: heidrun/scripts/run-tests.sh
#        or from heidrun/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HEIDRUN_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$HEIDRUN_DIR"
docker compose -f docker-compose.test.yml run --rm heidrun-test
