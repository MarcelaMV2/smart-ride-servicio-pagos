use bson::{doc, oid::ObjectId};
use chrono::Utc;
use mongodb::options::{FindOptions, UpdateOptions};
use crate::{db::MongoCtx, models::pago::{PagoDoc, CrearPagoDTO, ActualizarPagoDTO}};

#[derive(thiserror::Error, Debug)]
pub enum PagoError {
    #[error("No encontrado")]
    NotFound,
    #[error("Error BD: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("Id inválido")]
    BadId,
}

pub struct PagoRepo;

impl PagoRepo {
    pub async fn crear(ctx: &MongoCtx, dto: CrearPagoDTO) -> Result<PagoDoc, PagoError> {
        let now = Utc::now();
        let doc = PagoDoc {
            id: ObjectId::new(),
            id_viaje: dto.id_viaje,
            id_pagador: dto.id_pagador,
            id_receptor: dto.id_receptor,
            monto_centavos: dto.monto_centavos,
            estado: "pendiente".into(),
            creado_en: now,
            actualizado_en: now,
        };
        ctx.pagos.insert_one(&doc, None).await?;
        Ok(doc)
    }

    pub async fn listar(ctx: &MongoCtx, limit: i64) -> Result<Vec<PagoDoc>, PagoError> {
        let mut cursor = ctx.pagos.find(None, FindOptions::builder().limit(limit).build()).await?;
        let mut v = Vec::new();
        while let Some(item) = cursor.try_next().await.map_err(PagoError::Db)? {
            v.push(item);
        }
        Ok(v)
    }

    pub async fn obtener(ctx: &MongoCtx, id: &str) -> Result<PagoDoc, PagoError> {
        let oid = ObjectId::parse_str(id).map_err(|_| PagoError::BadId)?;
        let f = ctx.pagos.find_one(doc! { "_id": oid }, None).await?;
        f.ok_or(PagoError::NotFound)
    }

    pub async fn actualizar(ctx: &MongoCtx, id: &str, dto: ActualizarPagoDTO) -> Result<PagoDoc, PagoError> {
        let oid = ObjectId::parse_str(id).map_err(|_| PagoError::BadId)?;
        let mut set_doc = doc! { "actualizado_en": Utc::now() };
        if let Some(e) = dto.estado { set_doc.insert("estado", e); }
        if let Some(m) = dto.monto_centavos { set_doc.insert("monto_centavos", m); }

        ctx.pagos.update_one(doc!{"_id": oid}, doc!{"$set": set_doc}, UpdateOptions::default()).await?;
        Self::obtener(ctx, id).await
    }

    pub async fn eliminar(ctx: &MongoCtx, id: &str) -> Result<(), PagoError> {
        let oid = ObjectId::parse_str(id).map_err(|_| PagoError::BadId)?;
        let res = ctx.pagos.delete_one(doc!{"_id": oid}, None).await?;
        if res.deleted_count == 0 { return Err(PagoError::NotFound); }
        Ok(())
    }
}
