#!/usr/bin/env bash
# Geri: Run all tests in container (Phase 1; depends on postgres, mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: geri/scripts/run-tests.sh
#        or from geri/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm geri-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GERI_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$GERI_DIR"
docker compose -f docker-compose.test.yml run --rm geri-test
