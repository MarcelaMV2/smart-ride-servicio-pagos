mod config;
mod db;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use config::AppConfig;
use db::MongoCtx;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::pagos::crear,
    ),
    components(
        schemas(models::pago::PagoDoc, models::pago::CrearPagoDTO, models::pago::ActualizarPagoDTO)
    ),
    tags(
        (name = "Pagos", description = "CRUD de pagos")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cfg = AppConfig::from_env();
    let mongo = MongoCtx::new(&cfg.mongo_uri, &cfg.mongo_db, &cfg.mongo_col).await;

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(mongo.clone()))
            .service(routes::health::salud)
            .service(
                actix_web::web::scope("/api/pagos")
                    .service(routes::pagos::crear)
                    .service(routes::pagos::listar)
                    .service(routes::pagos::obtener)
                    .service(routes::pagos::actualizar)
                    .service(routes::pagos::eliminar)
            )
            .service(
                SwaggerUi::new("/api/pagos/docs/{_:.*}")
                    .url("/api/pagos/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(("0.0.0.0", cfg.port))?
    .run()
    .await
}
