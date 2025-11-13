FROM rust:latest

WORKDIR /app
# Copiamos todo el proyecto
COPY . .
# Compilamos en release
RUN cargo build --release
# Puerto del microservicio
EXPOSE 4000
# Ejecutamos el binario
CMD ["./target/release/smart-ride-servicio-pagos"]
