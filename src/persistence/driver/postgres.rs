use crate::config::PostgresPersistenceDiverConfig;
use crate::persistence::driver_trait::PersistenceDriver;
use crate::persistence::error::PersistenceError;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use sqlx::{Connection, Executor};
use tokio::sync::Mutex;

pub struct PostgresPersistenceDriver<PrimaryKey, Item> {
    storage: Arc<Mutex<HashMap<PrimaryKey, Item>>>,
    conn: Arc<Mutex<sqlx::PgConnection>>,
}

impl<PrimaryKey, Item> PostgresPersistenceDriver<PrimaryKey, Item> {
    pub async fn new_for_config(config: PostgresPersistenceDiverConfig) -> Self {
        Self {
            storage: Arc::new(Default::default()),
            conn: Arc::new(Mutex::new(sqlx::PgConnection::connect(config.connection_string.as_str()).await.unwrap()))
        }
    }
}

#[async_trait::async_trait]
impl<P, I> PersistenceDriver for PostgresPersistenceDriver<P, I>
where
    P: Send,
    I: Send + Clone,
{
    type PrimaryKey = P;
    type Item = I;

    async fn get_item(&self, id: Self::PrimaryKey) -> Result<Option<Self::Item>, PersistenceError> {
        unimplemented!()
    }

    async fn list_items(&self) -> Result<Vec<Self::Item>, PersistenceError> {
        unimplemented!()
    }

    async fn delete_item(&self, id: Self::PrimaryKey) -> Result<(), PersistenceError> {
        unimplemented!()
    }

    async fn add_item(
        &self,
        pk: Self::PrimaryKey,
        item: Self::Item,
    ) -> Result<(), PersistenceError> {
        unimplemented!()
    }
}
