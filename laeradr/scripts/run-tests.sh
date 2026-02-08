#!/usr/bin/env bash
# Læraðr: Run all tests in container.
# Usage: from repo root: laeradr/scripts/run-tests.sh
#        or from laeradr/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LAERADR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$LAERADR_DIR"
docker compose -f docker-compose.test.yml run --rm laeradr-test
