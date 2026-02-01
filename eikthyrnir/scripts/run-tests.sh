#!/usr/bin/env bash
# Eikthyrnir: Run all tests in container.
# Usage: from repo root: eikthyrnir/scripts/run-tests.sh
#        or from eikthyrnir/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EIKTHYRNIR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$EIKTHYRNIR_DIR"
docker compose -f docker-compose.test.yml run --rm eikthyrnir-test
