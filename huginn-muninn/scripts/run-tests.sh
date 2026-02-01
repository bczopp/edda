#!/usr/bin/env bash
# Huginn-Muninn: Run all tests in container (Phase 1; see docker-compose.test.yml for dependencies).
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: huginn-muninn/scripts/run-tests.sh
#        or from huginn-muninn/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm huginn-muninn-test cargo test <test_name>

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$HM_DIR"
docker compose -f docker-compose.test.yml run --rm huginn-muninn-test
