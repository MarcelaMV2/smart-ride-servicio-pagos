use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use utoipa::path;
use crate::{db::MongoCtx, models::pago::{CrearPagoDTO, ActualizarPagoDTO, PagoDoc}, services::mod::PagoRepo};

#[utoipa::path(post, path="/api/pagos/pagos", request_body=CrearPagoDTO, responses(
    (status=201, body=PagoDoc)
))]
#[post("/pagos")]
pub async fn crear(ctx: web::Data<MongoCtx>, body: web::Json<CrearPagoDTO>) -> impl Responder {
    match PagoRepo::crear(&ctx, body.0).await {
        Ok(doc) => HttpResponse::Created().json(doc),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[get("/pagos")]
pub async fn listar(ctx: web::Data<MongoCtx>) -> impl Responder {
    match PagoRepo::listar(&ctx, 50).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[get("/pagos/{id}")]
pub async fn obtener(ctx: web::Data<MongoCtx>, id: web::Path<String>) -> impl Responder {
    match PagoRepo::obtener(&ctx, &id).await {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[patch("/pagos/{id}")]
pub async fn actualizar(ctx: web::Data<MongoCtx>, id: web::Path<String>, body: web::Json<ActualizarPagoDTO>) -> impl Responder {
    match PagoRepo::actualizar(&ctx, &id, body.0).await {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[delete("/pagos/{id}")]
pub async fn eliminar(ctx: web::Data<MongoCtx>, id: web::Path<String>) -> impl Responder {
    match PagoRepo::eliminar(&ctx, &id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/pagos")
        .service(crear)
        .service(listar)
        .service(obtener)
        .service(actualizar)
        .service(eliminar)
}
