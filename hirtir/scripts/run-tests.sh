#!/usr/bin/env bash
# Hirtir: Run all tests in container.
# Runs: cargo test --release (see docker-compose.test.yml).
# Usage: from repo root: hirtir/scripts/run-tests.sh
#        or from hirtir/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HIRTIR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$HIRTIR_DIR"
docker compose -f docker-compose.test.yml run --rm hirtir-test
