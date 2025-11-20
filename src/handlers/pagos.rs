use actix_web::{web, HttpResponse, get, post, patch, delete};
use mongodb::Collection;
use mongodb::bson::{doc, DateTime as BsonDateTime};
use crate::db::AppState;
use crate::models::{Pago, CrearPagoRequest, ActualizarPagoRequest};
use chrono::Utc;
use uuid::Uuid;

/// Crear pago
#[utoipa::path(
    post,
    path = "/api/v1/pagos",
    tag = "Pagos",
    request_body = CrearPagoRequest,
    responses(
        (status = 201, description = "Pago creado", body = Pago),
        (status = 400, description = "Datos inválidos")
    )
)]
#[post("/pagos")]
pub async fn crear_pago(
    state: web::Data<AppState>,
    body: web::Json<CrearPagoRequest>,
) -> HttpResponse {
    println!("📥 Creando nuevo pago para viaje {}", body.id_viaje);
    
    let collection: Collection<Pago> = state.pagos.clone();
    
    let count = collection.count_documents(doc! {}).await.unwrap_or(0);
    let id_pago = (count + 1) as i32;
    
    let nuevo_pago = Pago {
        id_pago,
        id_viaje: body.id_viaje,
        id_pasajero: body.id_pasajero,
        id_conductor: body.id_conductor,
        desglose_costo: crate::services::CalculoService::calcular_costo_viaje(
            body.distancia_km,
            body.duracion_minutos,
            &crate::config::Settings::new(),
        ),
        metodo_pago: body.metodo_pago.clone(),
        tipo_pago: crate::models::TipoPago::Simulado,
        estado_pago: crate::models::EstadoPago::Completado,
        referencia_pago: Some(format!("REF-{}", Uuid::new_v4().to_string()[..8].to_uppercase())),
        fecha_pago: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    match collection.insert_one(nuevo_pago.clone()).await {
        Ok(_) => {
            println!("✅ Pago {} creado exitosamente", id_pago);
            HttpResponse::Created().json(nuevo_pago)
        }
        Err(e) => {
            eprintln!("❌ Error creando pago: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Error: {}", e)
            }))
        }
    }
}

/// Listar todos los pagos
#[utoipa::path(
    get,
    path = "/api/v1/pagos",
    tag = "Pagos",
    responses(
        (status = 200, description = "Lista de pagos", body = Vec<Pago>)
    )
)]
#[get("/pagos")]
pub async fn listar_pagos(state: web::Data<AppState>) -> HttpResponse {
    let collection: Collection<Pago> = state.pagos.clone();
    
    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut pagos = Vec::new();
            while cursor.advance().await.unwrap_or(false) {
                if let Ok(pago) = cursor.deserialize_current() {
                    pagos.push(pago);
                }
            }
            HttpResponse::Ok().json(pagos)
        }
        Err(e) => {
            eprintln!("❌ Error listando pagos: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Error: {}", e)
            }))
        }
    }
}

/// Obtener pago por ID
#[utoipa::path(
    get,
    path = "/api/v1/pagos/{id}",
    tag = "Pagos",
    params(
        ("id" = i32, Path, description = "ID del pago")
    ),
    responses(
        (status = 200, description = "Pago encontrado", body = Pago),
        (status = 404, description = "Pago no encontrado")
    )
)]
#[get("/pagos/{id}")]
pub async fn obtener_pago(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Pago> = state.pagos.clone();
    
    match collection.find_one(doc! { "id_pago": id }).await {
        Ok(Some(pago)) => HttpResponse::Ok().json(pago),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Pago {} no encontrado", id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Actualizar pago
#[utoipa::path(
    patch,
    path = "/api/v1/pagos/{id}",
    tag = "Pagos",
    params(
        ("id" = i32, Path, description = "ID del pago")
    ),
    request_body = ActualizarPagoRequest,
    responses(
        (status = 200, description = "Pago actualizado", body = Pago),
        (status = 404, description = "Pago no encontrado")
    )
)]
#[patch("/pagos/{id}")]
pub async fn actualizar_pago(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<ActualizarPagoRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Pago> = state.pagos.clone();
    
    let now_bson = BsonDateTime::from_millis(Utc::now().timestamp_millis());
    let mut update_doc = doc! { "$set": { "updated_at": now_bson } };
    
    if let Some(estado) = &body.estado_pago {
        update_doc.get_document_mut("$set").unwrap().insert("estado_pago", format!("{:?}", estado).to_lowercase());
    }
    
    match collection.update_one(doc! { "id_pago": id }, update_doc).await {
        Ok(result) if result.matched_count > 0 => {
            match collection.find_one(doc! { "id_pago": id }).await {
                Ok(Some(pago)) => HttpResponse::Ok().json(pago),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Pago {} no encontrado", id)
        }))
    }
}

/// Eliminar pago
#[utoipa::path(
    delete,
    path = "/api/v1/pagos/{id}",
    tag = "Pagos",
    params(
        ("id" = i32, Path, description = "ID del pago")
    ),
    responses(
        (status = 204, description = "Pago eliminado"),
        (status = 404, description = "Pago no encontrado")
    )
)]
#[delete("/pagos/{id}")]
pub async fn eliminar_pago(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Pago> = state.pagos.clone();
    
    match collection.delete_one(doc! { "id_pago": id }).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Pago {} no encontrado", id)
        }))
    }
}