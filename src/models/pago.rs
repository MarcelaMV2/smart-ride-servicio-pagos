use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MetodoPago {
    Efectivo,
    Tarjeta,
    #[serde(rename = "wallet")]
    Wallet,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoPago {
    Real,
    Simulado,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPago {
    Pendiente,
    Completado,
    Fallido,
    Reembolsado,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DesgloseCosto {
    pub tarifa_base: f64,
    pub distancia_km: f64,
    pub costo_por_km: f64,
    pub duracion_minutos: i32,
    pub costo_por_minuto: f64,
    pub subtotal: f64,
    pub impuestos: f64,
    pub descuentos: f64,
    pub total_final: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Pago {
    //  Usar i32 en lugar de String
    pub id_pago: i32,
    pub id_viaje: i32,
    pub id_pasajero: i32,
    pub id_conductor: i32,
    
    pub desglose_costo: DesgloseCosto,
    pub metodo_pago: MetodoPago,
    pub tipo_pago: TipoPago,
    pub estado_pago: EstadoPago,
    pub referencia_pago: Option<String>,
    
    pub fecha_pago: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearPagoRequest {
    pub id_viaje: i32,
    pub id_pasajero: i32,
    pub id_conductor: i32,
    pub distancia_km: f64,
    pub duracion_minutos: i32,
    pub metodo_pago: MetodoPago,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarPagoRequest {
    pub estado_pago: Option<EstadoPago>,
    pub metodo_pago: Option<MetodoPago>,
}
