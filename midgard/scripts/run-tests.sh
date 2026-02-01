#!/usr/bin/env bash
# Midgard: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: midgard/scripts/run-tests.sh
#        or from midgard/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIDGARD_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$MIDGARD_DIR"
docker compose -f docker-compose.test.yml run --rm midgard-test
