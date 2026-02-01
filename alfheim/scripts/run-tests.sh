#!/usr/bin/env bash
# Alfheim: Run all tests in container (Bun).
# Runs: bun test (see docker-compose.test.yml).
# Usage: from repo root: alfheim/scripts/run-tests.sh
#        or from alfheim/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ALFHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ALFHEIM_DIR"
docker compose -f docker-compose.test.yml run --rm alfheim-test
