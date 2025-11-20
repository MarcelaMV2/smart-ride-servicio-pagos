use mongodb::Database;
use mongodb::bson::{doc, Document};
use chrono::Utc;
use uuid::Uuid;
use crate::models::pago::{Pago, EstadoPago, MetodoPago, TipoPago};
use crate::services::calculo_service::{CalculoService};
use crate::config::Settings;
use crate::utils::AppError;

pub struct PagoService;

impl PagoService {
    /// Procesar pago de un viaje completado
    pub async fn procesar_pago(
        db: &Database,
        id_viaje: i32,
        id_pasajero: i32,
        id_conductor: i32,
        distancia_km: f64,
        duracion_minutos: i32,
        metodo_pago: MetodoPago,
        settings: &Settings,
    ) -> Result<Pago, AppError> {
        tracing::info!(
            "🔄 [PAGO_SERVICE] Procesando pago para viaje {}",
            id_viaje
        );
        
        // 1. Calcular costo
        let desglose = CalculoService::calcular_costo_viaje(
            distancia_km,
            duracion_minutos,
            settings,
        );
        
        // 2. Generar ID único
        let id_pago = Self::generar_id_pago(db).await?;
        
        // 3. Simular procesamiento de pago
        let (estado_pago, referencia_pago) = Self::simular_pago(&metodo_pago);
        
        // 4. Crear documento de pago
        let pago = Pago {
            id_pago,
            id_viaje,
            id_pasajero,
            id_conductor,
            desglose_costo: desglose,
            metodo_pago: metodo_pago.clone(),
            tipo_pago: TipoPago::Simulado,
            estado_pago: estado_pago.clone(),
            referencia_pago: Some(referencia_pago),
            fecha_pago: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // 5. Guardar en MongoDB
        let collection = db.collection::<Pago>("pagos");
        collection.insert_one(&pago).await?;
        
        tracing::info!(
            "✅ [PAGO_SERVICE] Pago {} creado exitosamente - Estado: {:?} - Total: {:.2} Bs",
            id_pago,
            estado_pago,
            pago.desglose_costo.total_final
        );
        
        Ok(pago)
    }
    
    /// Generar ID secuencial para pago
    async fn generar_id_pago(db: &Database) -> Result<i32, AppError> {
        let collection = db.collection::<Document>("counters");
        
        let filter = doc! { "_id": "pago_id" };
        let update = doc! { "$inc": { "seq": 1 } };
        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        
        let result = collection.find_one_and_update(filter, update).with_options(options).await?;
        
        match result {
            Some(doc) => {
                let seq = doc.get_i32("seq")
                    .map_err(|e| AppError::Database(format!("Error obteniendo secuencia: {}", e)))?;
                Ok(seq)
            }
            None => Err(AppError::Database("No se pudo generar ID de pago".to_string())),
        }
    }
    
    /// Simular procesamiento de pago
    fn simular_pago(metodo_pago: &MetodoPago) -> (EstadoPago, String) {
        // Generar referencia única
        let referencia = match metodo_pago {
            MetodoPago::Efectivo => format!("EF-{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
            MetodoPago::Tarjeta => format!("TC-{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
            MetodoPago::Wallet => format!("WL-{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
        };
        
        // Simular éxito (95% de probabilidad)
        let exito = rand::random::<f32>() < 0.95;
        
        let estado = if exito {
            EstadoPago::Completado
        } else {
            EstadoPago::Fallido
        };
        
        (estado, referencia)
    }
    
    /// Obtener pago por ID de viaje
    pub async fn obtener_por_viaje(
        db: &Database,
        id_viaje: i32,
    ) -> Result<Option<Pago>, AppError> {
        let collection = db.collection::<Pago>("pagos");
        let filter = doc! { "id_viaje": id_viaje };
        
        let pago = collection.find_one(filter).await?;
        Ok(pago)
    }
    
    /// Obtener pago por ID
    pub async fn obtener_por_id(
        db: &Database,
        id_pago: i32,
    ) -> Result<Option<Pago>, AppError> {
        let collection = db.collection::<Pago>("pagos");
        let filter = doc! { "id_pago": id_pago };
        
        let pago = collection.find_one(filter).await?;
        Ok(pago)
    }
    
    /// Listar pagos de un pasajero
    pub async fn listar_por_pasajero(
        db: &Database,
        id_pasajero: i32,
    ) -> Result<Vec<Pago>, AppError> {
        let collection = db.collection::<Pago>("pagos");
        let filter = doc! { "id_pasajero": id_pasajero };
        
        let mut cursor = collection.find(filter).await?;
        let mut pagos = Vec::new();
        
        while cursor.advance().await? {
            let pago = cursor.deserialize_current()?;
            pagos.push(pago);
        }
        
        Ok(pagos)
    }
}