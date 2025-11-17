FROM rust:latest as builder

WORKDIR /app

# Copiar archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear directorio src dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar código fuente real
COPY src ./src

# Compilar la aplicación final
RUN cargo build --release

# Etapa 2: Runtime - Imagen ligera para producción
FROM debian:bookworm-slim

# Instalar dependencias mínimas de runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado desde builder
COPY --from=builder /app/target/release/smart-ride-servicio-pagos /app/smart-ride-servicio-pagos

# Usuario no-root para seguridad
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

USER appuser

# Exponer puerto
EXPOSE 4000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:4000/health || exit 1

# Ejecutar aplicación
CMD ["./smart-ride-servicio-pagos"]