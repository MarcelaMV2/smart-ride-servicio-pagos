use crate::models::{Pago, Factura};
use mongodb::{options::ClientOptions, Client, Collection};
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
    println!("URI: {}", mongo_uri.replace("password123", "***"));  
    println!("Base de datos: {}", db_name);

    let mut client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("No se pudo parsear la URI de MongoDB");
    
    client_options.app_name = Some("smart-ride-servicio-pagos".to_string());

    let client = Client::with_options(client_options)
        .expect("No se pudo crear cliente de MongoDB");

    // Verificar conexión
    match client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await
    {
        Ok(_) => println!("Conectado exitosamente a MongoDB"),
        Err(e) => {
            eprintln!("Error conectando a MongoDB: {:?}", e);
            panic!("No se pudo conectar a MongoDB");
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