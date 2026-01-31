#!/bin/sh
# Test container entrypoint: run all tests in container.
# When DATABASE_URL is set (docker-compose), tests use that Postgres; otherwise
# testcontainers is used (requires Docker on host).
set -e
exec cargo test --release "$@"
