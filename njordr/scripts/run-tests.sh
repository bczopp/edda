#!/usr/bin/env bash
# Njörðr: Run all tests in container.
# Usage: from repo root: njordr/scripts/run-tests.sh
#        or from njordr/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NJORDR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$NJORDR_DIR"
docker compose -f docker-compose.test.yml run --rm njordr-test
