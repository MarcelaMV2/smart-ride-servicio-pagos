use actix_web::{web, HttpResponse, get, post, patch, delete};
use mongodb::Collection;
use mongodb::bson::doc;
use crate::db::AppState;
use crate::models::{Factura, CrearFacturaRequest, ActualizarFacturaRequest, EstadoFactura, DetalleViaje};
use chrono::Utc;

/// Crear factura
#[utoipa::path(
    post,
    path = "/api/v1/facturas",
    tag = "Facturas",
    request_body = CrearFacturaRequest,
    responses(
        (status = 201, description = "Factura creada", body = Factura)
    )
)]
#[post("/facturas")]
pub async fn crear_factura(
    state: web::Data<AppState>,
    body: web::Json<CrearFacturaRequest>,
) -> HttpResponse {
    let collection_facturas: Collection<Factura> = state.facturas.clone();
    let collection_pagos: Collection<crate::models::Pago> = state.pagos.clone();
    
    // Verificar que el pago existe
    let pago = match collection_pagos.find_one(doc! { "id_pago": body.id_pago }).await {
        Ok(Some(p)) => p,
        Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Pago {} no encontrado", body.id_pago)
        })),
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    };
    
    // Generar ID secuencial
    let count = collection_facturas.count_documents(doc! {}).await.unwrap_or(0);
    let id_factura = (count + 1) as i32;
    
    // Generar número de factura
    let numero_factura = format!("FACT-{}-{:05}", Utc::now().format("%Y%m%d"), id_factura);
    
    let nueva_factura = Factura {
        id_factura,
        id_pago: pago.id_pago,
        id_viaje: pago.id_viaje,
        id_pasajero: pago.id_pasajero,
        id_conductor: pago.id_conductor,
        detalle_viaje: DetalleViaje {
            origen: body.origen.clone(),
            destino: body.destino.clone(),
            distancia_km: pago.desglose_costo.distancia_km,
            duracion_minutos: pago.desglose_costo.duracion_minutos,
        },
        costo_total: pago.desglose_costo.total_final,
        estado_factura: EstadoFactura::Emitida,
        numero_factura,
        fecha_emision: Utc::now(),
        created_at: Utc::now(),
    };
    
    match collection_facturas.insert_one(nueva_factura.clone()).await {
        Ok(_) => HttpResponse::Created().json(nueva_factura),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Listar facturas
#[utoipa::path(
    get,
    path = "/api/v1/facturas",
    tag = "Facturas",
    responses(
        (status = 200, description = "Lista de facturas", body = Vec<Factura>)
    )
)]
#[get("/facturas")]
pub async fn listar_facturas(state: web::Data<AppState>) -> HttpResponse {
    let collection: Collection<Factura> = state.facturas.clone();
    
    match collection.find(doc! {}).await {
        Ok(mut cursor) => {
            let mut facturas = Vec::new();
            while cursor.advance().await.unwrap_or(false) {
                if let Ok(factura) = cursor.deserialize_current() {
                    facturas.push(factura);
                }
            }
            HttpResponse::Ok().json(facturas)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Obtener factura por ID
#[utoipa::path(
    get,
    path = "/api/v1/facturas/{id}",
    tag = "Facturas",
    params(
        ("id" = i32, Path, description = "ID de la factura")
    ),
    responses(
        (status = 200, description = "Factura encontrada", body = Factura),
        (status = 404, description = "Factura no encontrada")
    )
)]
#[get("/facturas/{id}")]
pub async fn obtener_factura(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Factura> = state.facturas.clone();
    
    match collection.find_one(doc! { "id_factura": id }).await {
        Ok(Some(factura)) => HttpResponse::Ok().json(factura),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Factura {} no encontrada", id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Obtener factura por número
#[utoipa::path(
    get,
    path = "/api/v1/facturas/numero/{numero}",
    tag = "Facturas",
    params(
        ("numero" = String, Path, description = "Número de factura")
    ),
    responses(
        (status = 200, description = "Factura encontrada", body = Factura),
        (status = 404, description = "Factura no encontrada")
    )
)]
#[get("/facturas/numero/{numero}")]
pub async fn obtener_factura_por_numero(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let numero = path.into_inner();
    let collection: Collection<Factura> = state.facturas.clone();
    
    match collection.find_one(doc! { "numero_factura": &numero }).await {
        Ok(Some(factura)) => HttpResponse::Ok().json(factura),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Factura {} no encontrada", numero)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Obtener factura por ID de pago
#[utoipa::path(
    get,
    path = "/api/v1/facturas/pago/{id_pago}",
    tag = "Facturas",
    params(
        ("id_pago" = i32, Path, description = "ID del pago")
    ),
    responses(
        (status = 200, description = "Factura encontrada", body = Factura),
        (status = 404, description = "Factura no encontrada")
    )
)]
#[get("/facturas/pago/{id_pago}")]
pub async fn obtener_factura_por_pago(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id_pago = path.into_inner();
    let collection: Collection<Factura> = state.facturas.clone();
    
    match collection.find_one(doc! { "id_pago": id_pago }).await {
        Ok(Some(factura)) => HttpResponse::Ok().json(factura),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Factura para pago {} no encontrada", id_pago)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Error: {}", e)
        }))
    }
}

/// Actualizar factura
#[utoipa::path(
    patch,
    path = "/api/v1/facturas/{id}",
    tag = "Facturas",
    params(
        ("id" = i32, Path, description = "ID de la factura")
    ),
    request_body = ActualizarFacturaRequest,
    responses(
        (status = 200, description = "Factura actualizada", body = Factura)
    )
)]
#[patch("/facturas/{id}")]
pub async fn actualizar_factura(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<ActualizarFacturaRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Factura> = state.facturas.clone();
    
    let mut update_doc = doc! {};
    
    if let Some(estado) = &body.estado_factura {
        update_doc.insert("estado_factura", format!("{:?}", estado).to_lowercase());
    }
    
    if update_doc.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No hay campos para actualizar"
        }));
    }
    
    match collection.update_one(doc! { "id_factura": id }, doc! { "$set": update_doc }).await {
        Ok(result) if result.matched_count > 0 => {
            match collection.find_one(doc! { "id_factura": id }).await {
                Ok(Some(factura)) => HttpResponse::Ok().json(factura),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Factura {} no encontrada", id)
        }))
    }
}

/// Eliminar factura
#[utoipa::path(
    delete,
    path = "/api/v1/facturas/{id}",
    tag = "Facturas",
    params(
        ("id" = i32, Path, description = "ID de la factura")
    ),
    responses(
        (status = 204, description = "Factura eliminada")
    )
)]
#[delete("/facturas/{id}")]
pub async fn eliminar_factura(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    let collection: Collection<Factura> = state.facturas.clone();
    
    match collection.delete_one(doc! { "id_factura": id }).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Factura {} no encontrada", id)
        }))
    }
}