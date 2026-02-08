#!/usr/bin/env bash
# Vedrfolnir: Run all tests in container.
# Usage: from repo root: vedrfolnir/scripts/run-tests.sh
#        or from vedrfolnir/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VEDRFOLNIR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$VEDRFOLNIR_DIR"
docker compose -f docker-compose.test.yml run --rm vedrfolnir-test
