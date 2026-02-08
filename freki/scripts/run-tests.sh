#!/usr/bin/env bash
# Freki: Run all tests in container (Phase 1; depends on qdrant, redis, mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: freki/scripts/run-tests.sh
#        or from freki/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm freki-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FREKI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$FREKI_DIR"
docker compose -f docker-compose.test.yml run --rm freki-test
