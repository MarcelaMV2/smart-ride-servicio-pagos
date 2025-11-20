use lapin::{
    options::*,
    BasicProperties,
    Channel,
};
use serde::Serialize;
use serde_json;
use crate::utils::AppError;

pub struct EventPublisher {
    channel: Channel,
    exchange: String,
}

impl EventPublisher {
    pub fn new(channel: Channel, exchange: String) -> Self {
        Self { channel, exchange }
    }
    
    /// Publicar evento genérico
    pub async fn publish<T: Serialize>(
        &self,
        routing_key: &str,
        event: &T,
    ) -> Result<(), AppError> {
        let payload = serde_json::to_vec(event)
            .map_err(|e| AppError::InternalServer(format!("Error serializando evento: {}", e)))?;
        
        self.channel
            .basic_publish(
                &self.exchange,
                routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2), // Persistent
            )
            .await?;
        
        tracing::info!(
            "📤 [PUBLISHER] Evento publicado: {} -> {}",
            routing_key,
            std::str::from_utf8(&payload).unwrap_or("invalid utf8")
        );
        
        Ok(())
    }
    
    /// Publicar evento de pago completado
    pub async fn publish_pago_completado(
        &self,
        id_pago: i32,
        id_viaje: i32,
        id_factura: i32,
        total_final: f64,
    ) -> Result<(), AppError> {
        #[derive(Serialize)]
        struct PagoCompletadoEvent {
            event_type: String,
            timestamp: String,
            data: PagoCompletadoData,
        }
        
        #[derive(Serialize)]
        struct PagoCompletadoData {
            id_pago: i32,
            id_viaje: i32,
            id_factura: i32,
            total_final: f64,
            estado_pago: String,
        }
        
        let event = PagoCompletadoEvent {
            event_type: "pago_completado".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data: PagoCompletadoData {
                id_pago,
                id_viaje,
                id_factura,
                total_final,
                estado_pago: "completado".to_string(),
            },
        };
        
        self.publish("payment.pago_completado", &event).await
    }
    
    /// Publicar evento de pago fallido
    pub async fn publish_pago_fallido(
        &self,
        id_viaje: i32,
        motivo: String,
    ) -> Result<(), AppError> {
        #[derive(Serialize)]
        struct PagoFallidoEvent {
            event_type: String,
            timestamp: String,
            data: PagoFallidoData,
        }
        
        #[derive(Serialize)]
        struct PagoFallidoData {
            id_viaje: i32,
            motivo: String,
            estado_pago: String,
        }
        
        let event = PagoFallidoEvent {
            event_type: "pago_fallido".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data: PagoFallidoData {
                id_viaje,
                motivo,
                estado_pago: "fallido".to_string(),
            },
        };
        
        self.publish("payment.pago_fallido", &event).await
    }
}