# Stage 1: Builder
FROM rust:latest AS builder

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker's layer caching
COPY Cargo.toml Cargo.lock ./

# Attempt to build dependencies to cache them
# This step will only rebuild if Cargo.toml or Cargo.lock changes
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy the rest of the application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runner
FROM debian:bookworm-slim AS runner 

# Install any necessary runtime dependencies if required by your application
# For a basic Rust binary, this might not be needed.
# RUN apt-get update && apt-get install -y <package_name> && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backendcompiler ./

# Set the entrypoint to run your application
CMD ["./backendcompiler"]