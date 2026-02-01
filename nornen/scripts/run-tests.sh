#!/usr/bin/env bash
# Nornen: Run all tests in container (Phase 1; see docker-compose.test.yml for dependencies).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: nornen/scripts/run-tests.sh
#        or from nornen/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm nornen-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NORNEN_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$NORNEN_DIR"
docker compose -f docker-compose.test.yml run --rm nornen-test
