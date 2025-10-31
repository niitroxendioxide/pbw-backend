FROM rust:1.75-slim

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar todo
COPY . .

# Compilar
RUN cargo build --release

# Exponer puerto
EXPOSE 60016

# Ejecutar
CMD ["sh", "-c", "cargo run --release"]