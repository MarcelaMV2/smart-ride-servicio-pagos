mod models;
mod db;
mod handlers;
mod config;
mod services;
mod events;
mod middleware;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use db::{init_db, AppState};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::{io, sync::Arc};
use mongodb::Database;
use crate::events::EventConsumer;
use crate::config::Settings;
use crate::utils::init_logger;

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
        handlers::facturas::obtener_factura_por_numero,
        handlers::facturas::obtener_factura_por_pago,
        handlers::facturas::actualizar_factura,
        handlers::facturas::eliminar_factura,
        handlers::health::health_check,
        handlers::health::info,
    ),
    components(
        schemas(
            models::Pago,
            models::CrearPagoRequest,
            models::ActualizarPagoRequest,
            models::DesgloseCosto,
            models::MetodoPago,
            models::EstadoPago,
            models::TipoPago,
            models::Factura,
            models::CrearFacturaRequest,
            models::ActualizarFacturaRequest,
            models::DetalleViaje,
            models::EstadoFactura,
            models::TransaccionFallida,
            handlers::health::HealthResponse,
            handlers::health::HealthChecks,
        )
    ),
    tags(
        (name = "Pagos", description = "Gestión de pagos de Smart Ride"),
        (name = "Facturas", description = "Gestión de facturas de Smart Ride"),
        (name = "Health", description = "Health checks y estado del servicio"),
        (name = "Info", description = "Información del servicio")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    init_logger();
    
    tracing::info!("========================================");
    tracing::info!("🚀 Iniciando Servicio de Pagos - Smart Ride");
    tracing::info!("========================================");
    tracing::info!("📦 Versión: {}", env!("CARGO_PKG_VERSION"));
    tracing::info!("🦀 Runtime: Rust + Actix-web");
    tracing::info!("========================================");
    
    let settings = Settings::new();
    tracing::info!("⚙️  Configuración cargada");
    tracing::info!("   - MongoDB: {}", settings.mongo_uri);
    tracing::info!("   - RabbitMQ: {}", settings.rabbitmq_url);
    tracing::info!("   - Tarifa Base: {:.2} Bs", settings.tarifa_base);
    tracing::info!("   - Costo/km: {:.2} Bs", settings.costo_por_km);
    tracing::info!("   - Costo/min: {:.2} Bs", settings.costo_por_minuto);
    
    tracing::info!("🔌 Conectando a MongoDB...");
    let state: AppState = init_db().await;
    tracing::info!("✅ Conexión a MongoDB establecida");
    
    let db_client = mongodb::Client::with_uri_str(&settings.mongo_uri)
        .await
        .expect("Error creando cliente MongoDB");
    let db: Database = db_client.database(&settings.mongo_db);
    let db_arc = Arc::new(db);
    
    tracing::info!("========================================");
    tracing::info!("🐰 Iniciando Consumer de RabbitMQ...");
    tracing::info!("========================================");
    
    let consumer = EventConsumer::new(
        &settings.rabbitmq_url,
        settings.rabbitmq_exchange.clone(),
        Arc::clone(&db_arc),
    )
    .await
    .expect("Error creando consumer de RabbitMQ");
    
    tokio::spawn(async move {
        if let Err(e) = consumer.start().await {
            tracing::error!("❌ Error en consumer de RabbitMQ: {:?}", e);
        }
    });
    
    tracing::info!("✅ Consumer de RabbitMQ iniciado");
    
    let openapi = ApiDoc::openapi();
    let bind_address = settings.bind_address();

    tracing::info!("========================================");
    tracing::info!("🌐 Servidor HTTP escuchando en: http://{}", bind_address);
    tracing::info!("📖 Swagger UI: http://localhost:{}/apidocs/", settings.server_port);
    tracing::info!("🏥 Health Check: http://localhost:{}/health", settings.server_port);
    tracing::info!("📊 API Base: http://localhost:{}/api/v1", settings.server_port);
    tracing::info!("========================================");
    tracing::info!("✅ Servicio listo para recibir solicitudes");
    tracing::info!("========================================");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let openapi = openapi.clone();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            
            //  HEALTH CHECK SIN PREFIJO (para Docker healthcheck)
            .service(handlers::health::health_check)
            
            // SCOPE /api/v1 - Todos los endpoints REST
            .service(
                web::scope("/api/v1")
                    // Info
                    .service(handlers::health::info)
                    
                    // Pagos
                    .service(handlers::pagos::crear_pago)
                    .service(handlers::pagos::listar_pagos)
                    .service(handlers::pagos::obtener_pago)
                    .service(handlers::pagos::actualizar_pago)
                    .service(handlers::pagos::eliminar_pago)
                    
                    // Facturas
                    .service(handlers::facturas::crear_factura)
                    .service(handlers::facturas::listar_facturas)
                    .service(handlers::facturas::obtener_factura)
                    .service(handlers::facturas::obtener_factura_por_numero)
                    .service(handlers::facturas::obtener_factura_por_pago)
                    .service(handlers::facturas::actualizar_factura)
                    .service(handlers::facturas::eliminar_factura)
            )
            
            // SWAGGER UI (sin prefijo)
            .service(
                SwaggerUi::new("/apidocs/{_:.*}")
                    .url("/api-docs/openapi.json", openapi),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}