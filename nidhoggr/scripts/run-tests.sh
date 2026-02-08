#!/usr/bin/env bash
# Nidhöggr: Run all tests in container (build context: repo root).
# Usage: from repo root: nidhoggr/scripts/run-tests.sh
#        or from nidhoggr/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NIDHOGGR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$NIDHOGGR_DIR"
docker compose -f docker-compose.test.yml run --rm nidhoggr-test
