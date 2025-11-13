use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetodoPago {
    Efectivo,
    Tarjeta,
    Billetera,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPago {
    Pendiente,
    Completado,
    Fallido,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DesgloseCosto {
    pub tarifa_base: f64,
    pub distancia_km: f64,
    pub tiempo_minutos: i32,
    pub subtotal: f64,
    pub propina: f64,
    pub total_final: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Pago {
    pub id_pago: String,
    pub id_viaje: String,
    pub id_pasajero: String,
    pub id_conductor: String,
    pub desglose_costo: DesgloseCosto,
    pub metodo_pago: MetodoPago,
    pub estado_pago: EstadoPago,
    pub fecha_pago: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearPagoRequest {
    pub id_viaje: String,
    pub id_pasajero: String,
    pub id_conductor: String,
    pub desglose_costo: DesgloseCosto,
    pub metodo_pago: MetodoPago,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarPagoRequest {
    pub estado_pago: Option<EstadoPago>,
    pub metodo_pago: Option<MetodoPago>,
    pub desglose_costo: Option<DesgloseCosto>,
}
