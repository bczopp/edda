#!/usr/bin/env bash
# Valkyries: Run all tests in container.
# Usage: from repo root: valkyries/scripts/run-tests.sh
#        or from valkyries/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VALKYRIES_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$VALKYRIES_DIR"
docker compose -f docker-compose.test.yml run --rm valkyries-test
