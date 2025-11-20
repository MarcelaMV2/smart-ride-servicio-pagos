# ==========================================
# Etapa 1: Builder - Compilar la aplicación
# ==========================================
FROM rust:latest AS builder
# FROM rust:1.91.1-alpine AS builder

WORKDIR /app

# Copiar archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear directorio src dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar código fuente real
COPY src ./src

# Forzar recompilación del binario
RUN touch src/main.rs

# Compilar la aplicación final
RUN cargo build --release && \
    ls -lh target/release/

# ==========================================
# Etapa 2: Runtime - Imagen ligera
# ==========================================
FROM debian:bookworm-slim

# Instalar dependencias mínimas de runtime (incluye file para verificación)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3 \
    file && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado desde builder
COPY --from=builder /app/target/release/smart-ride-servicio-pagos /app/smart-ride-servicio-pagos

# Verificar que el binario existe y es ejecutable
RUN ls -lh /app/smart-ride-servicio-pagos && \
    file /app/smart-ride-servicio-pagos && \
    chmod +x /app/smart-ride-servicio-pagos

# Usuario no-root para seguridad
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Exponer puerto
EXPOSE 4000

# Variables de entorno por defecto
ENV RUST_LOG=info \
    RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:4000/health || exit 1

# Ejecutar aplicación
CMD ["/app/smart-ride-servicio-pagos"]