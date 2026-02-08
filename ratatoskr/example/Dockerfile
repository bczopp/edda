FROM rust:latest as base

WORKDIR /app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

FROM base as builder

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./

# Create dummy source to build dependencies
RUN mkdir -p src/proto src/protocol src/messages && \
    echo "pub mod protocol; pub mod messages; pub mod proto;" > src/lib.rs && \
    echo "// Placeholder" > src/protocol/mod.rs && \
    echo "// Placeholder" > src/messages/mod.rs && \
    echo "// Placeholder" > src/proto/mod.rs && \
    mkdir -p proto && \
    echo "syntax = \"proto3\"; package ratatoskr;" > proto/ratatoskr.proto

# Build dependencies
RUN cargo build --release || true

# Copy actual source code
COPY src ./src
COPY proto ./proto
COPY tests ./tests

# Build the project
RUN cargo build --release

FROM base as test

# Copy dependency files
COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./

# Create dummy source to build dependencies
RUN mkdir -p src/proto src/protocol src/messages && \
    echo "pub mod protocol; pub mod messages; pub mod proto;" > src/lib.rs && \
    echo "// Placeholder" > src/protocol/mod.rs && \
    echo "// Placeholder" > src/messages/mod.rs && \
    echo "// Placeholder" > src/proto/mod.rs && \
    mkdir -p proto && \
    echo "syntax = \"proto3\"; package ratatoskr;" > proto/ratatoskr.proto

# Build dependencies
RUN cargo build --tests || true

# Copy actual source code
COPY src ./src
COPY proto ./proto
COPY tests ./tests

# Default command runs tests
CMD ["cargo", "test"]

FROM base as dev

# Copy everything for development
COPY . .

# Default command for interactive development
CMD ["bash"]
