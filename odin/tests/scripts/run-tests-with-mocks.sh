#!/bin/sh
# Start all mocks inside this container, then run Odin tests.
# Pro Projekt ein Container – Mocks laufen als Hintergrundprozesse.

set -e
BIN="${MOCKS_BIN:-/app/mocks_bin}"
export PATH="$BIN:$PATH"

# TCP-Mocks (Thor, Freki, Huginn, Muninn, Loki, Heimdall, Skuld) – ein Binary, verschiedene Ports
SERVICE_NAME=heimdall SERVICE_PORT=50051 "$BIN/odin-mock-services" &
SERVICE_NAME=thor    SERVICE_PORT=50052 "$BIN/odin-mock-services" &
SERVICE_NAME=freki   SERVICE_PORT=50053 "$BIN/odin-mock-services" &
SERVICE_NAME=huginn  SERVICE_PORT=50055 "$BIN/odin-mock-services" &
SERVICE_NAME=muninn  SERVICE_PORT=50056 "$BIN/odin-mock-services" &
SERVICE_NAME=loki    SERVICE_PORT=50057 "$BIN/odin-mock-services" &
SERVICE_NAME=skuld   SERVICE_PORT=50058 "$BIN/odin-mock-services" &

# gRPC-Mock für Geri (Einherjar, Responsibility, ProcessPrompt)
SERVICE_PORT=50054 "$BIN/grpc_geri" &

sleep 3

export THOR_URL="${THOR_URL:-http://127.0.0.1:50052}"
export FREKI_URL="${FREKI_URL:-http://127.0.0.1:50053}"
export GERI_URL="${GERI_URL:-http://127.0.0.1:50054}"
export HUGINN_URL="${HUGINN_URL:-http://127.0.0.1:50055}"
export MUNINN_URL="${MUNINN_URL:-http://127.0.0.1:50056}"
export LOKI_URL="${LOKI_URL:-http://127.0.0.1:50057}"
export HEIMDALL_URL="${HEIMDALL_URL:-http://127.0.0.1:50051}"
export SKULD_URL="${SKULD_URL:-http://127.0.0.1:50058}"

exec cargo test --release "$@"
