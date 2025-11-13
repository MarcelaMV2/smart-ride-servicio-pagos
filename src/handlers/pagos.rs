use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::doc;

use crate::db::AppState;
use crate::models::{
    Pago,
    CrearPagoRequest,
    ActualizarPagoRequest,
    EstadoPago,
};

/// aquí va crear_pago, listar_pagos, obtener_pago, actualizar_pago, eliminar_pago
/// (tal cual los tienes, solo ajustando los imports de arriba)
/// Crear un pago (POST /pagos)
#[utoipa::path(
    post,
    path = "/pagos",
    request_body = CrearPagoRequest,
    responses(
        (status = 201, description = "Pago creado", body = Pago),
        (status = 500, description = "Error interno")
    ),
    tag = "Pagos"
)]
#[post("/pagos")]
pub async fn crear_pago(
    data: web::Data<AppState>,
    body: web::Json<CrearPagoRequest>,
) -> impl Responder {
    let now = Utc::now();
    //let id_pago = Uuid::new_v4().to_string();
    let id_pago = chrono::Utc::now().timestamp_millis().to_string();

    let nuevo_pago = Pago {
        /* id: None, */
        id_pago,
        id_viaje: body.id_viaje.clone(),
        id_pasajero: body.id_pasajero.clone(),
        id_conductor: body.id_conductor.clone(),
        desglose_costo: body.desglose_costo.clone(),
        metodo_pago: body.metodo_pago.clone(),
        estado_pago: EstadoPago::Pendiente,
        fecha_pago: None,
        created_at: now,
        updated_at: now,
    };

    match data.pagos.insert_one(&nuevo_pago, None).await {
        Ok(_) => HttpResponse::Created().json(nuevo_pago),
        Err(e) => {
            eprintln!("Error insertando pago: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al crear el pago"
            }))
        }
    }
}

/// Listar todos los pagos (GET /pagos)
#[utoipa::path(
    get,
    path = "/pagos",
    responses(
        (status = 200, description = "Lista de pagos", body = [Pago]),
        (status = 500, description = "Error interno")
    ),
    tag = "Pagos"
)]
#[get("/pagos")]
pub async fn listar_pagos(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.pagos.find(None, None).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error listando pagos: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al listar pagos"
            }));
        }
    };

    let mut pagos: Vec<Pago> = Vec::new();
    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
        pagos.push(doc);
    }

    HttpResponse::Ok().json(pagos)
}

/// Obtener un pago por id_pago (GET /pagos/{id_pago})
#[utoipa::path(
    get,
    path = "/pagos/{id_pago}",
    params(
        ("id_pago" = String, Path, description = "UUID del pago")
    ),
    responses(
        (status = 200, description = "Pago encontrado", body = Pago),
        (status = 404, description = "Pago no encontrado"),
        (status = 500, description = "Error interno")
    ),
    tag = "Pagos"
)]
#[get("/pagos/{id_pago}")]
pub async fn obtener_pago(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_pago = path.into_inner();
    let filtro = doc! { "id_pago": &id_pago };

    match data.pagos.find_one(filtro, None).await {
        Ok(Some(pago)) => HttpResponse::Ok().json(pago),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "mensaje": "Pago no encontrado"
        })),
        Err(e) => {
            eprintln!("Error buscando pago: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al buscar pago"
            }))
        }
    }
}

/// Actualizar un pago (PATCH /pagos/{id_pago})
#[utoipa::path(
    patch,
    path = "/pagos/{id_pago}",
    request_body = ActualizarPagoRequest,
    params(
        ("id_pago" = String, Path, description = "UUID del pago")
    ),
    responses(
        (status = 200, description = "Pago actualizado", body = Pago),
        (status = 404, description = "Pago no encontrado"),
        (status = 500, description = "Error interno")
    ),
    tag = "Pagos"
)]
#[patch("/pagos/{id_pago}")]
pub async fn actualizar_pago(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<ActualizarPagoRequest>,
) -> impl Responder {
    let id_pago = path.into_inner();
    let filtro = doc! { "id_pago": &id_pago };

    // 1) Buscar el pago
    let mut pago = match data.pagos.find_one(filtro.clone(), None).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "mensaje": "Pago no encontrado"
            }));
        }
        Err(e) => {
            eprintln!("Error buscando pago para actualizar: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al buscar pago"
            }));
        }
    };

    // 2) Aplicar cambios en el struct
    if let Some(estado) = &body.estado_pago {
        pago.estado_pago = estado.clone();
        if let EstadoPago::Completado = estado {
            pago.fecha_pago = Some(Utc::now());
        }
    }
    if let Some(metodo) = &body.metodo_pago {
        pago.metodo_pago = metodo.clone();
    }
    if let Some(desglose) = &body.desglose_costo {
        pago.desglose_costo = desglose.clone();
    }
    pago.updated_at = Utc::now();

    // 3) Guardar el pago actualizado
    match data
        .pagos
        .replace_one(filtro, &pago, None)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(pago),
        Err(e) => {
            eprintln!("Error actualizando pago: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al actualizar pago"
            }))
        }
    }
}

/// Eliminar un pago (DELETE /pagos/{id_pago}`)
#[utoipa::path(
    delete,
    path = "/pagos/{id_pago}",
    params(
        ("id_pago" = String, Path, description = "UUID del pago")
    ),
    responses(
        (status = 200, description = "Pago eliminado"),
        (status = 404, description = "Pago no encontrado"),
        (status = 500, description = "Error interno")
    ),
    tag = "Pagos"
)]
#[delete("/pagos/{id_pago}")]
pub async fn eliminar_pago(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_pago = path.into_inner();
    let filtro = doc! { "id_pago": &id_pago };

    match data.pagos.delete_one(filtro, None).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                HttpResponse::NotFound().json(serde_json::json!({
                    "mensaje": "Pago no encontrado"
                }))
            } else {
                HttpResponse::Ok().json(serde_json::json!({
                    "mensaje": "Pago eliminado correctamente"
                }))
            }
        }
        Err(e) => {
            eprintln!("Error eliminando pago: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al eliminar pago"
            }))
        }
    }
}
