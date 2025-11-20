use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TransaccionFallida {
    pub id_transaccion: i32,
    pub id_pago: Option<i32>,
    pub id_viaje: i32,
    pub tipo_error: String,
    pub mensaje_error: String,
    pub codigo_error: Option<String>,
    pub intentos: i32,
    pub fecha_intento: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TransaccionFallida {
    pub fn new(
        id_transaccion: i32,
        id_viaje: i32,
        tipo_error: String,
        mensaje_error: String,
    ) -> Self {
        Self {
            id_transaccion,
            id_pago: None,
            id_viaje,
            tipo_error,
            mensaje_error,
            codigo_error: None,
            intentos: 1,
            fecha_intento: Utc::now(),
            created_at: Utc::now(),
        }
    }
}