#!/usr/bin/env bash
# Odin: Run all tests in container (Phase 1; uses run-tests-with-mocks.sh inside container).
# Usage: from repo root: odin/scripts/run-tests.sh
#        or from odin/: ./scripts/run-tests.sh
# Single test: docker compose -f docker-compose.test.yml run --rm odin-test cargo test <test_name>
# See also: run-tests-in-container.sh (same behavior).

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ODIN_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ODIN_DIR"
docker compose -f docker-compose.test.yml run --rm odin-test
