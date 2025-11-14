# Smart Ride – Microservicio de Pagos

Microservicio de **Pagos y Facturación** del sistema distribuido **Smart Ride**.  
Se encarga de registrar pagos de viajes, calcular montos finales y generar facturas asociadas, utilizando **Rust + Actix Web** y **MongoDB**.

---

## Rol dentro de la arquitectura

Este microservicio forma parte de un sistema de reservas de transporte tipo “Smart Ride”:

- Recibe eventos / peticiones HTTP cuando un viaje ha sido completado.
- Registra el pago con su desglose de costos.
- Permite consultar, actualizar y eliminar pagos.
- Gestiona la colección de **facturas** asociadas a los pagos.
- Expone documentación **Swagger / OpenAPI** para pruebas con Postman o navegador.

---

## Tecnologías principales

- **Lenguaje:** Rust
- **Framework web:** Actix Web
- **Documentación API:** utoipa + Swagger UI
- **Base de datos:** MongoDB
  - Colección `pagos`
  - Colección `facturas`
- **Contenerización:** Docker + Docker Compose

---

## Estructura del proyecto

```text
smart-ride-servicio-pagos/
├── src/
│   ├── main.rs          // Punto de entrada: inicia Actix Web, configura rutas y Swagger
│   ├── db.rs            // Conexión a MongoDB y definición de AppState (colecciones pagos/facturas)
│   │
│   ├── models/          // Módulo de modelos y DTOs
│   │   ├── mod.rs       // Re-exporta los modelos (pub mod pago; pub mod factura; ...)
│   │   ├── pago.rs      // Modelo Pago + enums relacionados al pago
│   │   └── factura.rs   // Modelo Factura + estructuras de datos_pasajero, detalle_viaje, etc.
│   │
│   ├── handlers/        // Módulo de controladores HTTP (endpoints REST)
│   │   ├── mod.rs       // Re-exporta los handlers (pub mod pagos; pub mod facturas; ...)
│   │   ├── pagos.rs     // Endpoints CRUD para pagos (/pagos)
│   │   └── facturas.rs  // Endpoints CRUD para facturas (/facturas)
│
├── Cargo.toml           // Nombre del paquete, dependencias (actix-web, mongodb, utoipa, etc.)
├── Cargo.lock           // Versión exacta de dependencias (lo genera Cargo)
├── .env                 // Variables de entorno locales (MONGO_URI, SERVER_PORT, etc.)
├── .env.example         // Plantilla de variables de entorno para el equipo
├── Dockerfile           // Imagen del microservicio de pagos en modo release
├── README.md            // Descripción del microservicio, cómo correrlo y cómo usar los endpoints
└── .gitignore           // Archivos y carpetas que no se suben al repo (target/, .env, etc.)
```


---

## Ejecución local

- Ejecutar el microservicio 

```
cargo run
```

- Abrir en swagger

```
http://localhost:4000/apidocs/
```

- Probar los endpoints

```
GET  http://localhost:4000/pagos
POST http://localhost:4000/pagos
PATCH http://localhost:4000/pagos/id
DELETE http://localhost:4000/pagos/id

GET  http://localhost:4000/facturas
POST http://localhost:4000/facturas
PATCH http://localhost:4000/facturas/id
DELETE http://localhost:4000/facturas/id
```

## 🐳 Ejecución con Docker

### **Prerrequisitos**

- Docker >= 20.10
- Docker Compose >= 2.0

### **Iniciar servicios**

```bash
# Build y start
docker compose up -d --build

# Ver logs en tiempo real
docker compose logs -f pagos-service

# Verificar estado
docker compose ps
```

### **Verificar funcionamiento**

```bash
# Health check
curl http://localhost:4000/health

# Swagger UI
http://localhost:4000/apidocs/

# Crear un pago de prueba
curl -X POST http://localhost:4000/pagos \
  -H "Content-Type: application/json" \
  -d '{
    "id_viaje": "123e4567-e89b-12d3-a456-426614174000",
    "monto_base": 25.50,
    "monto_adicional": 5.00,
    "metodo_pago": "tarjeta"
  }'
```

### **Detener servicios**

```bash
# Detener sin eliminar datos
docker compose stop

# Detener y eliminar contenedores
docker compose down

# Detener y eliminar TODO (incluye datos de MongoDB)
docker compose down -v
```

## 🌐 Puertos Utilizados

| Servicio | Puerto Host | Puerto Interno | Descripción |
|----------|-------------|----------------|-------------|
| **API Pagos** | `4000` | `4000` | API REST + Swagger |
| **MongoDB** | `27018` | `27017` | Base de datos |

> ⚠️ **Nota**: MongoDB se expone en el puerto `27018` del host para evitar conflictos con otros servicios que usen el puerto estándar `27017`.

## 🔗 Conectar desde otros servicios Docker

Si necesitas conectar otro servicio al servicio de pagos:

### Opción A: Usar la misma red Docker

```yaml
# En el docker-compose.yml del otro servicio
networks:
  default:
    external: true
    name: smart-ride-pagos-network
```

### Opción B: Conectar vía host

```yaml
# Conectar a MongoDB desde otro servicio
MONGO_HOST: host.docker.internal  # Windows/Mac
MONGO_HOST: 172.17.0.1           # Linux
MONGO_PORT: 27018
```

## 🧪 Desarrollo Local (sin Docker)

```bash
# 1. Asegurarse que MongoDB esté corriendo
# En Linux/Mac:
sudo systemctl start mongodb
# En Windows:
net start MongoDB

# 2. Configurar .env para desarrollo local
MONGO_URI=mongodb://localhost:27017
SERVER_PORT=4000

# 3. Ejecutar aplicación
cargo run
```

## 🐛 Troubleshooting

### Error: "Cannot connect to MongoDB"

```bash
# Verificar que MongoDB esté saludable
docker compose exec mongo-pagos mongosh --eval "db.adminCommand('ping')"

# Ver logs de MongoDB
docker compose logs mongo-pagos

# Reiniciar MongoDB
docker compose restart mongo-pagos
```

### Error: "Address already in use"

```bash
# Verificar qué proceso usa el puerto 4000
sudo lsof -i :4000  # Linux/Mac
netstat -ano | findstr :4000  # Windows

# Cambiar el puerto en .env y docker-compose.yml
SERVER_PORT=4001
```

### Rebuild completo

```bash
# Limpiar todo y reconstruir
docker compose down -v
docker compose build --no-cache
docker compose up -d
```


