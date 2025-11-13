mod models;
mod db;
mod handlers;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use db::{init_db, AppState};
use dotenvy::dotenv;
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::pagos::crear_pago,
        handlers::pagos::listar_pagos,
        handlers::pagos::obtener_pago,
        handlers::pagos::actualizar_pago,
        handlers::pagos::eliminar_pago,
        handlers::facturas::crear_factura,
        handlers::facturas::listar_facturas,
        handlers::facturas::obtener_factura,
        handlers::facturas::actualizar_factura,
        handlers::facturas::eliminar_factura,
    ),
    components(
        schemas(
            models::Pago,
            models::CrearPagoRequest,
            models::ActualizarPagoRequest,
            models::DesgloseCosto,
            models::MetodoPago,
            models::EstadoPago,
            models::Factura,
            models::CrearFacturaRequest,
            models::ActualizarFacturaRequest,
            models::DatosPasajero,
            models::DetalleViaje,
            models::EstadoFactura
        )
    ),
    tags(
        (name = "Pagos", description = "Gestión de pagos de Smart Ride"),
        (name = "Facturas", description = "Gestión de facturas de Smart Ride")
    )
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let state: AppState = init_db().await;

    let openapi = ApiDoc::openapi();

    println!("Servicio de Pagos escuchando en http://localhost:4000");
    println!("Swagger UI en             http://localhost:4000/apidocs/");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let openapi = openapi.clone();

        App::new()
            .wrap(cors)
            .app_data(actix_web::web::Data::new(state.clone()))
            // pagos
            .service(handlers::pagos::crear_pago)
            .service(handlers::pagos::listar_pagos)
            .service(handlers::pagos::obtener_pago)
            .service(handlers::pagos::actualizar_pago)
            .service(handlers::pagos::eliminar_pago)
            // facturas
            .service(handlers::facturas::crear_factura)
            .service(handlers::facturas::listar_facturas)
            .service(handlers::facturas::obtener_factura)
            .service(handlers::facturas::actualizar_factura)
            .service(handlers::facturas::eliminar_factura)
            // swagger
            .service(
                SwaggerUi::new("/apidocs/{_:.*}")
                    .url("/api-docs/openapi.json", openapi),
            )
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await

}
