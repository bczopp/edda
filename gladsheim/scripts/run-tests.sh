#!/usr/bin/env bash
# Gladsheim: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: gladsheim/scripts/run-tests.sh
#        or from gladsheim/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GLADSHEIM_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$GLADSHEIM_DIR"
docker compose -f docker-compose.test.yml run --rm gladsheim-test
