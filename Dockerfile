# Stage 1: Build
FROM rust:1.75-slim as builder

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar manifiestos
COPY Cargo.toml Cargo.lock ./

# Crear dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar código fuente
COPY . .

# Build real
RUN touch src/main.rs && \
    cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copiar el binario (ajusta el nombre según tu Cargo.toml)
COPY --from=builder /app/target/release/* /app/

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 60016

ENV RUST_LOG=info
ENV PORT=60016

# Ejecuta el único binario que exista
CMD ["/bin/sh", "-c", "exec /app/*"]