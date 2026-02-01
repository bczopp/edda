#!/usr/bin/env bash
# Thor: Run all tests in container (Phase 1 / Phase 15).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: thor/scripts/run-tests.sh
#        or from thor/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm thor-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THOR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$THOR_DIR"
docker compose -f docker-compose.test.yml run --rm thor-test
