#!/usr/bin/env bash
# Jotunheim: Run all tests in container.
# Usage: from repo root: jotunheim/scripts/run-tests.sh
#        or from jotunheim/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JOTUNHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$JOTUNHEIM_DIR"
docker compose -f docker-compose.test.yml run --rm jotunheim-test
