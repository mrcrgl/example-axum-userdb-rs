use crate::models::user::User;
use crate::persistence::driver::in_memory::InMemoryPersistenceDriver;
use crate::persistence::driver_trait::PersistenceDriver;

pub struct UserService {
    persistence_driver: Box<dyn PersistenceDriver<Item = User, PrimaryKey = uuid::Uuid>>,
}

impl UserService {
    pub fn new() -> UserService {
        UserService {
            persistence_driver: Box::new(InMemoryPersistenceDriver::<uuid::Uuid, User>::new()),
        }
    }

    pub async fn add(&self) -> Result<User, super::error::ServiceError> {
        unimplemented!()
    }

    pub async fn list(&self) -> Result<Vec<User>, super::error::ServiceError> {
        unimplemented!()
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<User>, super::error::ServiceError> {
        unimplemented!()
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), super::error::ServiceError> {
        unimplemented!()
    }
}
