use mongodb::Database;
use mongodb::bson::{doc, Document};
use chrono::Utc;
use crate::models::factura::{Factura, EstadoFactura, DetalleViaje};
use crate::models::pago::{Pago, EstadoPago};
use crate::utils::AppError;

pub struct FacturaService;

impl FacturaService {
    /// Generar factura desde un pago completado
    pub async fn generar_factura(
        db: &Database,
        pago: &Pago,
        origen: String,
        destino: String,
    ) -> Result<Factura, AppError> {
        // Validar que el pago esté completado
        if pago.estado_pago != EstadoPago::Completado {
            return Err(AppError::BadRequest(
                "Solo se pueden generar facturas para pagos completados".to_string()
            ));
        }
        
        tracing::info!(
            "📄 [FACTURA_SERVICE] Generando factura para pago {}",
            pago.id_pago
        );
        
        // 1. Generar ID único
        let id_factura = Self::generar_id_factura(db).await?;
        
        // 2. Generar número de factura
        let numero_factura = Self::generar_numero_factura(id_factura);
        
        // 3. Crear detalle del viaje
        let detalle_viaje = DetalleViaje {
            origen,
            destino,
            distancia_km: pago.desglose_costo.distancia_km,
            duracion_minutos: pago.desglose_costo.duracion_minutos,
        };
        
        // 4. Crear factura
        let factura = Factura {
            id_factura,
            id_pago: pago.id_pago,
            id_viaje: pago.id_viaje,
            id_pasajero: pago.id_pasajero,
            id_conductor: pago.id_conductor,
            detalle_viaje,
            costo_total: pago.desglose_costo.total_final,
            estado_factura: EstadoFactura::Emitida,
            numero_factura,
            fecha_emision: Utc::now(),
            created_at: Utc::now(),
        };
        
        // 5. Guardar en MongoDB
        let collection = db.collection::<Factura>("facturas");
        collection.insert_one(&factura).await?;
        
        tracing::info!(
            "✅ [FACTURA_SERVICE] Factura {} generada - Número: {} - Total: {:.2} Bs",
            id_factura,
            factura.numero_factura,
            factura.costo_total
        );
        
        Ok(factura)
    }
    
    /// Generar ID secuencial para factura
    async fn generar_id_factura(db: &Database) -> Result<i32, AppError> {
        let collection = db.collection::<Document>("counters");
        
        let filter = doc! { "_id": "factura_id" };
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
            None => Err(AppError::Database("No se pudo generar ID de factura".to_string())),
        }
    }
    
    /// Generar número de factura (formato: FACT-YYYYMMDD-XXXXX)
    fn generar_numero_factura(id_factura: i32) -> String {
        let fecha = Utc::now().format("%Y%m%d");
        format!("FACT-{}-{:05}", fecha, id_factura)
    }
    
    /// Obtener factura por ID de pago
    pub async fn obtener_por_pago(
        db: &Database,
        id_pago: i32,
    ) -> Result<Option<Factura>, AppError> {
        let collection = db.collection::<Factura>("facturas");
        let filter = doc! { "id_pago": id_pago };
        
        let factura = collection.find_one(filter).await?;
        Ok(factura)
    }
    
    /// Obtener factura por ID
    pub async fn obtener_por_id(
        db: &Database,
        id_factura: i32,
    ) -> Result<Option<Factura>, AppError> {
        let collection = db.collection::<Factura>("facturas");
        let filter = doc! { "id_factura": id_factura };
        
        let factura = collection.find_one(filter).await?;
        Ok(factura)
    }
    
    /// Listar facturas de un pasajero
    pub async fn listar_por_pasajero(
        db: &Database,
        id_pasajero: i32,
    ) -> Result<Vec<Factura>, AppError> {
        let collection = db.collection::<Factura>("facturas");
        let filter = doc! { "id_pasajero": id_pasajero };
        
        let mut cursor = collection.find(filter).await?;
        let mut facturas = Vec::new();
        
        while cursor.advance().await? {
            let factura = cursor.deserialize_current()?;
            facturas.push(factura);
        }
        
        Ok(facturas)
    }
}