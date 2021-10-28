use anyhow::Result;
use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct DB {
    pub conn: Arc<DatabaseConnection>,
}

impl DB {
    pub(crate) async fn new(address: &str) -> Result<Self> {
        Ok(DB {
            conn: Arc::new(Database::connect(address).await?),
        })
    }
}
