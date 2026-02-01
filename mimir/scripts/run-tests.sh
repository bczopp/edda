#!/usr/bin/env bash
# Mimir: Run all tests in container (Phase 1; see docker-compose.test.yml for dependencies).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: mimir/scripts/run-tests.sh
#        or from mimir/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm mimir-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIMIR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$MIMIR_DIR"
docker compose -f docker-compose.test.yml run --rm mimir-test
