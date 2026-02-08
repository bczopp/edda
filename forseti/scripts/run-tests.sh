#!/usr/bin/env bash
# Forseti: Run all tests in container.
# Usage: from repo root: forseti/scripts/run-tests.sh
#        or from forseti/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORSETI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$FORSETI_DIR"
docker compose -f docker-compose.test.yml run --rm forseti-test
