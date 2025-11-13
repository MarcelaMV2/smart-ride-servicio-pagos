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
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EstadoFactura {
    Emitida,
    Enviada,
    Anulada,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Factura {
    pub id_factura: String,
    pub numero_factura: String,
    pub id_pago: String,
    pub id_viaje: String,
    pub datos_pasajero: DatosPasajero,
    pub detalle_viaje: DetalleViaje,
    pub estado_factura: EstadoFactura,
    pub fecha_emision: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearFacturaRequest {
    pub numero_factura: String,
    pub id_pago: String,
    pub id_viaje: String,
    pub datos_pasajero: DatosPasajero,
    pub detalle_viaje: DetalleViaje,
    pub estado_factura: Option<EstadoFactura>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarFacturaRequest {
    pub estado_factura: Option<EstadoFactura>,
    pub datos_pasajero: Option<DatosPasajero>,
    pub detalle_viaje: Option<DetalleViaje>,
}
