#!/usr/bin/env bash
# Skuld: Run all tests in container (Phase 1; depends on postgres, mock-odin).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: skuld/scripts/run-tests.sh
#        or from skuld/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm skuld-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKULD_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$SKULD_DIR"
docker compose -f docker-compose.test.yml run --rm skuld-test
