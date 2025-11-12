use actix_web::{get, HttpResponse, Responder};

#[get("/salud")]
pub async fn salud() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"estado":"ok"}))
}
