#!/usr/bin/env bash
# Loki: Run all tests in container (Phase 1; depends on mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: loki/scripts/run-tests.sh
#        or from loki/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm loki-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOKI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$LOKI_DIR"
docker compose -f docker-compose.test.yml run --rm loki-test
