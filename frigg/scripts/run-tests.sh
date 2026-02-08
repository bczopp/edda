#!/usr/bin/env bash
# Frigg: Run all tests in container.
# Usage: from repo root: frigg/scripts/run-tests.sh
#        or from frigg/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FRIGG_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$FRIGG_DIR"
docker compose -f docker-compose.test.yml run --rm frigg-test
