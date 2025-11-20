use lapin::{
    options::*,
    types::FieldTable,
    Channel, Connection, Consumer,
};
use futures::StreamExt;
use crate::events::handlers::EventHandlers;
use crate::utils::AppError;
use mongodb::Database;
use std::sync::Arc;

pub struct EventConsumer {
    connection: Connection,
    exchange: String,
    db: Arc<Database>,
}

impl EventConsumer {
    pub async fn new(
        rabbitmq_url: &str,
        exchange: String,
        db: Arc<Database>,
    ) -> Result<Self, AppError> {
        tracing::info!("🐰 Conectando a RabbitMQ: {}", rabbitmq_url);
        
        let connection = Connection::connect(rabbitmq_url, lapin::ConnectionProperties::default())
            .await
            .map_err(|e| AppError::RabbitMQ(format!("Error conectando a RabbitMQ: {}", e)))?;
        
        tracing::info!("✅ Conexión a RabbitMQ establecida");
        
        Ok(Self {
            connection,
            exchange,
            db,
        })
    }
    
    /// Iniciar consumidor de eventos
    pub async fn start(&self) -> Result<(), AppError> {
        // Crear canal
        let channel = self.connection.create_channel().await?;
        
        tracing::info!("✅ Canal de RabbitMQ creado");
        
        // Declarar exchange
        channel
            .exchange_declare(
                &self.exchange,
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
        
        tracing::info!("✅ Exchange declarado: {}", self.exchange);
        
        // Declarar cola
        let queue_name = "pagos_queue";
        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
        
        tracing::info!("✅ Cola declarada: {}", queue_name);
        
        // Binding para evento ride.viaje_completado
        channel
            .queue_bind(
                queue_name,
                &self.exchange,
                "ride.viaje_completado",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        tracing::info!("🔗 Cola '{}' enlazada con routing key 'ride.viaje_completado'", queue_name);
        
        // Crear consumidor
        let mut consumer = channel
            .basic_consume(
                queue_name,
                "pagos_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        tracing::info!("👂 Consumer iniciado. Esperando mensajes...");
        
        // Clonar referencias necesarias
        let db = Arc::clone(&self.db);
        let channel_clone = channel.clone();
        let exchange_clone = self.exchange.clone();
        
        // Procesar mensajes
        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                match delivery {
                    Ok(delivery) => {
                        let routing_key = delivery.routing_key.as_str();
                        
                        tracing::info!(
                            "📨 [CONSUMER] Mensaje recibido - Routing Key: {}",
                            routing_key
                        );
                        
                        // Procesar mensaje
                        match Self::process_message(
                            &db,
                            &channel_clone,
                            &exchange_clone,
                            routing_key,
                            &delivery.data,
                        ).await {
                            Ok(_) => {
                                // ACK el mensaje
                                if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                                    tracing::error!("❌ Error haciendo ACK: {:?}", e);
                                } else {
                                    tracing::info!("✅ Mensaje procesado y confirmado");
                                }
                            }
                            Err(e) => {
                                tracing::error!("❌ Error procesando mensaje: {:?}", e);
                                // NACK el mensaje (requeue)
                                if let Err(nack_err) = delivery.nack(BasicNackOptions {
                                    requeue: true,
                                    ..Default::default()
                                }).await {
                                    tracing::error!("❌ Error haciendo NACK: {:?}", nack_err);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ Error recibiendo mensaje: {:?}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Procesar mensaje según routing key
    async fn process_message(
        db: &Database,
        channel: &Channel,
        exchange: &str,
        routing_key: &str,
        data: &[u8],
    ) -> Result<(), AppError> {
        match routing_key {
            "ride.viaje_completado" => {
                EventHandlers::handle_viaje_completado(db, channel, exchange, data).await
            }
            _ => {
                tracing::warn!("⚠️ Routing key no reconocido: {}", routing_key);
                Ok(())
            }
        }
    }
}