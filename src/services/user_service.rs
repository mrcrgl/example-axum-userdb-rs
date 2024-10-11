use crate::config::UserServiceConfig;
use crate::models::user::User;
use crate::persistence::driver::load_driver_for_config;
use crate::persistence::driver_trait::PersistenceDriver;

pub struct UserService {
    persistence_driver: Box<dyn PersistenceDriver<Item = User, PrimaryKey = uuid::Uuid> + Send + Sync>,
}

impl UserService {
    pub async fn new_for_config(config: UserServiceConfig) -> UserService {
        UserService {
            persistence_driver: load_driver_for_config(config.persistence).await,
        }
    }

    pub async fn add(&self, input: AddUserInput) -> Result<User, super::error::ServiceError> {
        let user = User {
            id: uuid::Uuid::new_v4(),
            username: input.username,
            email: input.email,
            created_at: chrono::Utc::now(),
        };

        self.persistence_driver
            .add_item(user.id, user.clone())
            .await?;

        Ok(user)
    }

    pub async fn list(&self) -> Result<Vec<User>, super::error::ServiceError> {
        let items = self.persistence_driver.list_items().await?;
        Ok(items)
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<User>, super::error::ServiceError> {
        let item = self.persistence_driver.get_item(id).await?;
        Ok(item)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), super::error::ServiceError> {
        self.persistence_driver.delete_item(id).await?;
        Ok(())
    }
}

pub struct AddUserInput {
    pub username: String,
    pub email: String,
}
