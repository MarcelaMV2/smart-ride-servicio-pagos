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


