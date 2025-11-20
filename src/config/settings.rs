use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    // Server
    pub server_port: u16,
    pub server_host: String,
    
    // MongoDB
    pub mongo_uri: String,
    pub mongo_db: String,
    
    // RabbitMQ
    pub rabbitmq_url: String,
    pub rabbitmq_exchange: String,
    
    // JWT
    pub jwt_secret: String,
    
    // Users Service
    pub users_service_url: String,
    
    // Tarifas
    pub tarifa_base: f64,
    pub costo_por_km: f64,
    pub costo_por_minuto: f64,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        
        Self {
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "4000".to_string())
                .parse()
                .expect("SERVER_PORT debe ser un número"),
            
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            mongo_uri: env::var("MONGO_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            
            mongo_db: env::var("MONGO_DB")
                .unwrap_or_else(|_| "smart_ride_pagos".to_string()),
            
            rabbitmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
            
            rabbitmq_exchange: env::var("RABBITMQ_EXCHANGE")
                .unwrap_or_else(|_| "smart_ride_exchange".to_string()),
            
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET debe estar configurado"),
            
            users_service_url: env::var("USERS_SERVICE_URL")
                .unwrap_or_else(|_| "http://users-service:3001".to_string()),
            
            tarifa_base: env::var("TARIFA_BASE")
                .unwrap_or_else(|_| "10.0".to_string())
                .parse()
                .unwrap_or(10.0),
            
            costo_por_km: env::var("COSTO_POR_KM")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()
                .unwrap_or(2.0),
            
            costo_por_minuto: env::var("COSTO_POR_MINUTO")
                .unwrap_or_else(|_| "0.5".to_string())
                .parse()
                .unwrap_or(0.5),
        }
    }
    
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}