use actix_web::{web, HttpResponse, get};
use serde::{Deserialize, Serialize};
use mongodb::Database;
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: String,
    pub checks: HealthChecks,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthChecks {
    pub database: String,
    pub rabbitmq: String,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Servicio saludable", body = HealthResponse)
    )
)]
#[get("/health")]
pub async fn health_check(_db: web::Data<Database>) -> HttpResponse {
    let db_status = "ok";
    let rabbitmq_status = "ok";
    
    let response = HealthResponse {
        status: if db_status == "ok" && rabbitmq_status == "ok" {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        service: "Smart Ride - Pagos Service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
        checks: HealthChecks {
            database: db_status.to_string(),
            rabbitmq: rabbitmq_status.to_string(),
        },
    };
    
    HttpResponse::Ok().json(response)
}

/// Info endpoint
#[utoipa::path(
    get,
    path = "/api/v1/info",
    tag = "Info",
    responses(
        (status = 200, description = "Información del servicio")
    )
)]
#[get("/info")]
pub async fn info() -> HttpResponse {
    #[derive(Serialize, ToSchema)]
    struct InfoResponse {
        service: String,
        version: String,
        description: String,
        endpoints: Vec<String>,
    }
    
    let response = InfoResponse {
        service: "Smart Ride - Pagos Service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Microservicio de pagos y facturación".to_string(),
        endpoints: vec![
            "GET /health".to_string(),
            "GET /api/v1/info".to_string(),
            "GET /api/v1/pagos".to_string(),
            "GET /api/v1/pagos/{id}".to_string(),
            "POST /api/v1/pagos".to_string(),
            "PATCH /api/v1/pagos/{id}".to_string(),
            "DELETE /api/v1/pagos/{id}".to_string(),
            "GET /api/v1/facturas".to_string(),
            "GET /api/v1/facturas/{id}".to_string(),
            "GET /api/v1/facturas/numero/{numero}".to_string(),
            "GET /api/v1/facturas/pago/{id_pago}".to_string(),
            "POST /api/v1/facturas".to_string(),
            "PATCH /api/v1/facturas/{id}".to_string(),
            "DELETE /api/v1/facturas/{id}".to_string(),
        ],
    };
    
    HttpResponse::Ok().json(response)
}