use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PagoDoc {
    #[schema(example = "64dbf7c9e1a1f9c1a2b3c4d5")]
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[schema(example = "UUID-VIAJE")]
    pub id_viaje: String,

    #[schema(example = "UUID-PERSONA-PAGADOR")]
    pub id_pagador: String,

    #[schema(example = "UUID-PERSONA-RECEPTOR")]
    pub id_receptor: String,

    #[schema(example = 2550, description = "En centavos (BOB)")]
    pub monto_centavos: i64,

    #[schema(example = "pendiente")]
    pub estado: String, // pendiente|aprobado|fallido|reembolsado

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub creado_en: DateTime<Utc>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub actualizado_en: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CrearPagoDTO {
    pub id_viaje: String,
    pub id_pagador: String,
    pub id_receptor: String,
    pub monto_centavos: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActualizarPagoDTO {
    pub estado: Option<String>,       // aprobado, fallido, reembolsado
    pub monto_centavos: Option<i64>,  // por si hay ajuste
}
