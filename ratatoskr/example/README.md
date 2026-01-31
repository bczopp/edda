# Ratatoskr Example Implementation

This is an example implementation of the Ratatoskr Business Protocol following the implementation plan.

## Overview

Ratatoskr is a WebSocket-based transport protocol for secure communication between user devices and Yggdrasil. It handles connection management, message transport, and security - but NOT business logic. Business requests use a generic `BUSINESS_REQUEST` with payload that is interpreted by the receiving services (Nornen, Heidrun, etc.).

## Implementation Status

Following the implementation plan phases:

- [x] Phase 1: Projekt-Setup
- [x] Phase 2: Protocol-Definition
- [x] Phase 3: Message-Serialization
- [x] Phase 4: Message-Validation
- [x] Phase 5: Connection-Protocol
- [x] Phase 6: Security-Features
- [x] Phase 7: Documentation & Examples
- [x] Phase 8: Testing

**Status**: ✅ All phases completed

## Building

**WICHTIG**: Alle Builds und Tests müssen in Docker-Containern ausgeführt werden. Es wird kein lokales Rust benötigt.

### Windows (PowerShell)

```powershell
# Build
.\docker-build.ps1

# Tests
.\docker-test.ps1

# Interactive Development
.\docker-dev.ps1
```

### Linux/Mac

```bash
# Make scripts executable
chmod +x docker-build.sh docker-test.sh docker-dev.sh

# Build
./docker-build.sh

# Tests
./docker-test.sh

# Interactive Development
./docker-dev.sh
```

### Docker Compose

```bash
# Run tests
docker-compose -f docker-compose.test.yml run --rm test

# Start interactive development container
docker-compose -f docker-compose.test.yml run --rm dev
```

### Direct Docker Commands

```bash
# Build
docker build -t ratatoskr-example:latest --target builder -f Dockerfile .

# Run tests
docker build -t ratatoskr-example:test --target test -f Dockerfile .
docker run --rm ratatoskr-example:test

# Interactive development
docker build -t ratatoskr-example:dev --target dev -f Dockerfile .
docker run -it --rm -v ${PWD}:/app -w /app ratatoskr-example:dev
```

## Structure

- `src/protocol/` - Protocol implementation
  - `serializer.rs` - Message serialization/deserialization
  - `validator.rs` - Message validation
  - `connection.rs` - Connection protocol
  - `security.rs` - Security features (signing, nonce management)
- `src/messages/` - Message type definitions
  - `request.rs` - Request helper functions
  - `response.rs` - Response helper functions
- `proto/` - Protocol Buffer definitions
- `tests/` - Integration tests
- `examples/` - Usage examples
- `docs/` - Protocol documentation

## Examples

Run the example programs:

```bash
# Client example
docker-compose -f docker-compose.test.yml run --rm dev cargo run --example client_example

# Server example
docker-compose -f docker-compose.test.yml run --rm dev cargo run --example server_example
```

## Documentation

See `docs/PROTOCOL.md` for detailed protocol documentation.
