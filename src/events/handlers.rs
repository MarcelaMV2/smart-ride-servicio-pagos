use mongodb::Database;
use lapin::Channel;
use serde::Deserialize;
use crate::services::{PagoService, FacturaService};
use crate::models::pago::MetodoPago;
use crate::events::publisher::EventPublisher;
use crate::config::Settings;
use crate::utils::AppError;

#[derive(Debug, Deserialize)]
struct ViajeCompletadoEvent {
    // ✅ SIMPLIFICADO: Sin anidamiento
    id_viaje: i32,
    id_pasajero: i32,
    id_conductor: i32,
    origen: String,
    destino: String,
    distancia_km: f64,
    duracion_minutos: i32,
    estado_viaje: String,
    metodo_pago: Option<String>,
    fecha_inicio: String,
    fecha_fin: String,
}

pub struct EventHandlers;

impl EventHandlers {
    /// Handler para evento: ride.viaje_completado
    pub async fn handle_viaje_completado(
        db: &Database,
        channel: &Channel,
        exchange: &str,
        data: &[u8],
    ) -> Result<(), AppError> {
        tracing::info!("============================================================");
        tracing::info!("🎯 [HANDLER] handle_viaje_completado INICIADO");
        tracing::info!("============================================================");
        
        // PARSEAR DIRECTAMENTE (sin campo 'data' anidado)
        let viaje_data: ViajeCompletadoEvent = serde_json::from_slice(data)
            .map_err(|e| AppError::BadRequest(format!("Error parseando evento: {}", e)))?;
        
        tracing::info!(
            "📦 Datos del viaje completado:\n   - ID Viaje: {}\n   - ID Pasajero: {}\n   - ID Conductor: {}\n   - Origen: {}\n   - Destino: {}\n   - Distancia: {:.2} km\n   - Duración: {} min",
            viaje_data.id_viaje,
            viaje_data.id_pasajero,
            viaje_data.id_conductor,
            viaje_data.origen,
            viaje_data.destino,
            viaje_data.distancia_km,
            viaje_data.duracion_minutos
        );
        
        // 2. Determinar método de pago
        let metodo_pago = match viaje_data.metodo_pago.as_deref() {
            Some("efectivo") => MetodoPago::Efectivo,
            Some("tarjeta") => MetodoPago::Tarjeta,
            Some("wallet") => MetodoPago::Wallet,
            _ => MetodoPago::Efectivo,
        };
        
        // 3. Cargar configuración
        let settings = Settings::new();
        
        // 4. Procesar pago
        tracing::info!("💰 Procesando pago para viaje {}", viaje_data.id_viaje);
        
        let pago_result = PagoService::procesar_pago(
            db,
            viaje_data.id_viaje,
            viaje_data.id_pasajero,
            viaje_data.id_conductor,
            viaje_data.distancia_km,
            viaje_data.duracion_minutos,
            metodo_pago,
            &settings,
        ).await;
        
        // 5. Crear publisher
        let publisher = EventPublisher::new(channel.clone(), exchange.to_string());
        
        match pago_result {
            Ok(pago) => {
                tracing::info!(
                    "✅ Pago {} procesado exitosamente - Total: {:.2} Bs",
                    pago.id_pago,
                    pago.desglose_costo.total_final
                );
                
                // 6. Generar factura
                tracing::info!("📄 Generando factura para pago {}", pago.id_pago);
                
                match FacturaService::generar_factura(
                    db,
                    &pago,
                    viaje_data.origen.clone(),
                    viaje_data.destino.clone(),
                ).await {
                    Ok(factura) => {
                        tracing::info!(
                            "✅ Factura {} generada - Número: {}",
                            factura.id_factura,
                            factura.numero_factura
                        );
                        
                        // 7. Publicar evento de pago completado
                        if let Err(e) = publisher.publish_pago_completado(
                            pago.id_pago,
                            pago.id_viaje,
                            factura.id_factura,
                            pago.desglose_costo.total_final,
                        ).await {
                            tracing::error!("❌ Error publicando evento pago_completado: {:?}", e);
                        } else {
                            tracing::info!("📤 [PUBLISHER] Evento publicado: payment.pago_completado");
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ Error generando factura: {:?}", e);
                        // Continuar aunque falle la factura (el pago está registrado)
                    }
                }
            }
            Err(e) => {
                tracing::error!("❌ Error procesando pago: {:?}", e);
                
                // Publicar evento de pago fallido
                if let Err(pub_err) = publisher.publish_pago_fallido(
                    viaje_data.id_viaje,
                    format!("Error procesando pago: {}", e),
                ).await {
                    tracing::error!("❌ Error publicando evento pago_fallido: {:?}", pub_err);
                } else {
                    tracing::info!("📤 [PUBLISHER] Evento publicado: payment.pago_fallido");
                }
                
                return Err(e);
            }
        }
        
        tracing::info!("============================================================");
        tracing::info!("🏁 [HANDLER] handle_viaje_completado FINALIZADO");
        tracing::info!("============================================================");
        
        Ok(())
    }
}