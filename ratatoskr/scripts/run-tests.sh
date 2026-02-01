#!/usr/bin/env bash
# Ratatoskr: Run all tests in container.
# Usage: from repo root: ratatoskr/scripts/run-tests.sh
#        or from ratatoskr/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RATATOSKR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$RATATOSKR_DIR"
docker compose -f docker-compose.test.yml run --rm ratatoskr-test
