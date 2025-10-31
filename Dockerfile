# Stage 1: Builder
FROM rust:bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

RUN cargo build --release

# Stage 2: Runner (misma base que builder)
FROM debian:bookworm-slim AS runner

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backendcompiler ./

EXPOSE 60016

ENV RUST_LOG=info
ENV PORT=60016

CMD ["./backendcompiler"]