#!/usr/bin/env bash
# Ragnarok: Run all tests in container.
# Usage: from repo root: ragnarok/scripts/run-tests.sh
#        or from ragnarok/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RAGNAROK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$RAGNAROK_DIR"
docker compose -f docker-compose.test.yml run --rm ragnarok-test
