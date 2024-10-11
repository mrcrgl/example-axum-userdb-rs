use crate::persistence::driver_trait::PersistenceDriver;
use crate::persistence::error::PersistenceError;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InMemoryPersistenceDriver<PrimaryKey, Item>
where
    PrimaryKey: Hash,
{
    storage: Arc<Mutex<HashMap<PrimaryKey, Item>>>,
}

impl<P, I> InMemoryPersistenceDriver<P, I>
where
    P: Send + Hash,
    I: Send,
{
    pub fn new() -> Self {
        InMemoryPersistenceDriver {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl<P, I> PersistenceDriver for InMemoryPersistenceDriver<P, I>
where
    P: Send + Hash + Eq,
    I: Send + Clone,
{
    type PrimaryKey = P;
    type Item = I;

    async fn get_item(&self, id: Self::PrimaryKey) -> Result<Option<Self::Item>, PersistenceError> {
        let guard = self.storage.lock().await;
        Ok(guard.get(&id).cloned())
    }

    async fn list_items(&self) -> Result<Vec<Self::Item>, PersistenceError> {
        let guard = self.storage.lock().await;
        Ok(guard.values().cloned().collect())
    }

    async fn delete_item(&self, id: Self::PrimaryKey) -> Result<(), PersistenceError> {
        let mut guard = self.storage.lock().await;
        guard.remove(&id);
        Ok(())
    }

    async fn add_item(&self, pk: Self::PrimaryKey, item: Self::Item) -> Result<(), PersistenceError> {
        let mut guard = self.storage.lock().await;
        guard.insert(pk, item);
        Ok(())
    }
}
