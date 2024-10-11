use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}
