#!/usr/bin/env bash
# Yggdrasil: Run all tests in container (Elixir).
# Runs: mix test (see docker-compose.test.yml).
# Usage: from repo root: yggdrasil/scripts/run-tests.sh
#        or from yggdrasil/: ./scripts/run-tests.sh

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
YGGDRASIL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$YGGDRASIL_DIR"
docker compose -f docker-compose.test.yml run --rm yggdrasil-test
