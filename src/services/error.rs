use crate::persistence;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("failed to call persistence operation: {0}")]
    Persistence(#[from] persistence::error::PersistenceError),
}
