#!/usr/bin/env bash
# Bifrost: Run all tests in container (Phase 1.2.1 / Phase 20).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: bifrost/scripts/run-tests.sh
#        or from bifrost/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm bifrost-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIFROST_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$BIFROST_DIR"
docker compose -f docker-compose.test.yml run --rm bifrost-test
