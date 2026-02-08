#!/usr/bin/env bash
# Heimdall: Run all tests in container (Phase 1; depends on Postgres).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: heimdall/scripts/run-tests.sh
#        or from heimdall/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm heimdall-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HEIMDALL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$HEIMDALL_DIR"
docker compose -f docker-compose.test.yml run --rm heimdall-test
