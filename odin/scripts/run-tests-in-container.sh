#!/usr/bin/env bash
# Run all Odin tests inside the container (no local Rust/cargo required).
# From repo: run from <odin-dir> or pass odin-dir as first argument.
set -e
ODIN_DIR="${1:-$(cd "$(dirname "$0")/.." && pwd)}"
cd "$ODIN_DIR"
docker compose -f docker-compose.test.yml run --rm odin-test
