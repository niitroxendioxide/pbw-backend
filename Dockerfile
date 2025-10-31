# Stage 1: Build
FROM rust:1.75-alpine as builder

# Instalar dependencias de compilación
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

WORKDIR /app

# Copiar manifiestos
COPY Cargo.toml Cargo.lock ./

# Dummy build para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar código real
COPY . .

# Build final
RUN touch src/main.rs && \
    cargo build --release

# Stage 2: Runtime
FROM alpine:latest

RUN apk add --no-cache \
    ca-certificates \
    libgcc

RUN adduser -D -u 1000 appuser

WORKDIR /app

COPY --from=builder /app/target/release/paintbloatware-rust-backend /app/server

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 60016

ENV RUST_LOG=info
ENV PORT=60016

CMD ["/app/server"]
