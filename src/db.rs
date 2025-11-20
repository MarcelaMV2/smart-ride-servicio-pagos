use crate::models::{Pago, Factura};
use mongodb::{options::ClientOptions, Client, Collection, bson::doc};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pagos: Collection<Pago>,
    pub facturas: Collection<Factura>,
}

pub async fn init_db() -> AppState {
    let mongo_uri = env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = env::var("MONGO_DB")
        .unwrap_or_else(|_| "smart_ride_pagos".to_string());

    println!("Intentando conectar a MongoDB...");
    println!("Base de datos: {}", db_name);

    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("No se pudo parsear la URI de MongoDB");

    let client = Client::with_options(client_options)
        .expect("No se pudo crear cliente de MongoDB");

    match tokio::time::timeout(
        std::time::Duration::from_secs(10),
        client.database("admin").run_command(doc! {"ping": 1})
    ).await {
        Ok(Ok(_)) => println!("Conectado exitosamente a MongoDB"),
        Ok(Err(e)) => {
            eprintln!("Error al conectar a MongoDB: {:?}", e);
            panic!("No se pudo conectar a MongoDB");
        }
        Err(_) => {
            eprintln!("Timeout al conectar a MongoDB (>10s)");
            panic!("Timeout de conexión a MongoDB");
        }
    }

    let db = client.database(&db_name);
    let pagos_collection: Collection<Pago> = db.collection("pagos");
    let facturas_collection: Collection<Factura> = db.collection("facturas");

    println!("Colecciones listas: pagos, facturas");

    AppState {
        pagos: pagos_collection,
        facturas: facturas_collection,
    }
}