use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::doc;

use crate::db::AppState;
use crate::models::{
    Factura,
    CrearFacturaRequest,
    ActualizarFacturaRequest,
    EstadoFactura,
};

/// aquí va crear_factura, listar_facturas, obtener_factura, actualizar_factura, eliminar_factura
/// Crear una factura (POST /facturas)
#[utoipa::path(
    post,
    path = "/facturas",
    request_body = CrearFacturaRequest,
    responses(
        (status = 201, description = "Factura creada", body = Factura),
        (status = 500, description = "Error interno")
    ),
    tag = "Facturas"
)]
#[post("/facturas")]
pub async fn crear_factura(
    data: web::Data<AppState>,
    body: web::Json<CrearFacturaRequest>,
) -> impl Responder {
    let now = Utc::now();
    let id_factura = format!("FAC-{}", Utc::now().timestamp_millis());

    let estado = body.estado_factura.clone().unwrap_or(EstadoFactura::Emitida);

    let nueva_factura = Factura {
        id_factura,
        numero_factura: body.numero_factura.clone(),
        id_pago: body.id_pago.clone(),
        id_viaje: body.id_viaje.clone(),
        datos_pasajero: body.datos_pasajero.clone(),
        detalle_viaje: body.detalle_viaje.clone(),
        estado_factura: estado,
        fecha_emision: now,
        created_at: now,
        updated_at: now,
    };

    match data.facturas.insert_one(&nueva_factura, None).await {
        Ok(_) => HttpResponse::Created().json(nueva_factura),
        Err(e) => {
            eprintln!("Error insertando factura: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al crear la factura"
            }))
        }
    }
}

/// Listar todas las facturas (GET /facturas)
#[utoipa::path(
    get,
    path = "/facturas",
    responses(
        (status = 200, description = "Lista de facturas", body = [Factura]),
        (status = 500, description = "Error interno")
    ),
    tag = "Facturas"
)]
#[get("/facturas")]
pub async fn listar_facturas(data: web::Data<AppState>) -> impl Responder {
    let mut cursor = match data.facturas.find(None, None).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error listando facturas: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al listar facturas"
            }));
        }
    };

    let mut facturas: Vec<Factura> = Vec::new();
    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
        facturas.push(doc);
    }

    HttpResponse::Ok().json(facturas)
}

/// Obtener una factura por id_factura (GET /facturas/{id_factura})
#[utoipa::path(
    get,
    path = "/facturas/{id_factura}",
    params(
        ("id_factura" = String, Path, description = "ID de la factura")
    ),
    responses(
        (status = 200, description = "Factura encontrada", body = Factura),
        (status = 404, description = "Factura no encontrada"),
        (status = 500, description = "Error interno")
    ),
    tag = "Facturas"
)]
#[get("/facturas/{id_factura}")]
pub async fn obtener_factura(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_factura = path.into_inner();
    let filtro = doc! { "id_factura": &id_factura };

    match data.facturas.find_one(filtro, None).await {
        Ok(Some(factura)) => HttpResponse::Ok().json(factura),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "mensaje": "Factura no encontrada"
        })),
        Err(e) => {
            eprintln!("Error buscando factura: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al buscar factura"
            }))
        }
    }
}

/// Actualizar una factura (PATCH /facturas/{id_factura})
#[utoipa::path(
    patch,
    path = "/facturas/{id_factura}",
    request_body = ActualizarFacturaRequest,
    params(
        ("id_factura" = String, Path, description = "ID de la factura")
    ),
    responses(
        (status = 200, description = "Factura actualizada", body = Factura),
        (status = 404, description = "Factura no encontrada"),
        (status = 500, description = "Error interno")
    ),
    tag = "Facturas"
)]
#[patch("/facturas/{id_factura}")]
pub async fn actualizar_factura(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<ActualizarFacturaRequest>,
) -> impl Responder {
    let id_factura = path.into_inner();
    let filtro = doc! { "id_factura": &id_factura };

    let mut factura = match data.facturas.find_one(filtro.clone(), None).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "mensaje": "Factura no encontrada"
            }));
        }
        Err(e) => {
            eprintln!("Error buscando factura para actualizar: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al buscar factura"
            }));
        }
    };

    if let Some(estado) = &body.estado_factura {
        factura.estado_factura = estado.clone();
    }
    if let Some(datos) = &body.datos_pasajero {
        factura.datos_pasajero = datos.clone();
    }
    if let Some(detalle) = &body.detalle_viaje {
        factura.detalle_viaje = detalle.clone();
    }

    factura.updated_at = Utc::now();

    match data.facturas.replace_one(filtro, &factura, None).await {
        Ok(_) => HttpResponse::Ok().json(factura),
        Err(e) => {
            eprintln!("Error actualizando factura: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al actualizar factura"
            }))
        }
    }
}

/// Eliminar una factura (DELETE /facturas/{id_factura}`)
#[utoipa::path(
    delete,
    path = "/facturas/{id_factura}",
    params(
        ("id_factura" = String, Path, description = "ID de la factura")
    ),
    responses(
        (status = 200, description = "Factura eliminada"),
        (status = 404, description = "Factura no encontrada"),
        (status = 500, description = "Error interno")
    ),
    tag = "Facturas"
)]
#[delete("/facturas/{id_factura}")]
pub async fn eliminar_factura(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_factura = path.into_inner();
    let filtro = doc! { "id_factura": &id_factura };

    match data.facturas.delete_one(filtro, None).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                HttpResponse::NotFound().json(serde_json::json!({
                    "mensaje": "Factura no encontrada"
                }))
            } else {
                HttpResponse::Ok().json(serde_json::json!({
                    "mensaje": "Factura eliminada correctamente"
                }))
            }
        }
        Err(e) => {
            eprintln!("Error eliminando factura: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "mensaje": "Error al eliminar factura"
            }))
        }
    }
}