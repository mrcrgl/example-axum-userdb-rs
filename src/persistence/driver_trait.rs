#[async_trait::async_trait]
pub trait PersistenceDriver {
    type PrimaryKey: Send;
    type Item: Send;
    async fn get_item(
        &self,
        id: Self::PrimaryKey,
    ) -> Result<Option<Self::Item>, super::error::PersistenceError>;
    async fn list_items(&self) -> Result<Vec<Self::Item>, super::error::PersistenceError>;
    async fn delete_item(&self, id: Self::PrimaryKey)
        -> Result<(), super::error::PersistenceError>;
    async fn add_item(
        &self,
        pk: Self::PrimaryKey,
        item: Self::Item,
    ) -> Result<(), super::error::PersistenceError>;
}
