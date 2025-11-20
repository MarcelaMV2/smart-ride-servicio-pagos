use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DatosPasajero {
    pub nombre_completo: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DetalleViaje {
    pub origen: String,
    pub destino: String,
    pub distancia_km: f64,
    pub duracion_minutos: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoFactura {
    Emitida,
    Anulada,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Factura {
    //  Usar i32
    pub id_factura: i32,
    pub id_pago: i32,
    pub id_viaje: i32,
    pub id_pasajero: i32,
    pub id_conductor: i32,
    
    pub detalle_viaje: DetalleViaje,
    pub costo_total: f64,
    pub estado_factura: EstadoFactura,
    pub numero_factura: String,
    
    pub fecha_emision: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearFacturaRequest {
    pub id_pago: i32,
    pub origen: String,
    pub destino: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarFacturaRequest {
    pub estado_factura: Option<EstadoFactura>,
}
