#!/usr/bin/env bash
# Asgard: Run all tests in container.
# Usage: from repo root: asgard/scripts/run-tests.sh
#        or from asgard/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ASGARD_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ASGARD_DIR"
docker compose -f docker-compose.test.yml run --rm asgard-test
