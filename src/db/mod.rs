use mongodb::{Client, Database, Collection};
use crate::models::pago::PagoDoc;

#[derive(Clone)]
pub struct MongoCtx {
    pub db: Database,
    pub pagos: Collection<PagoDoc>,
}

impl MongoCtx {
    pub async fn new(uri: &str, db_name: &str, col: &str) -> Self {
        let client = Client::with_uri_str(uri).await.expect("No conecta Mongo");
        let db = client.database(db_name);
        let pagos = db.collection::<PagoDoc>(col);
        Self { db, pagos }
    }
}
