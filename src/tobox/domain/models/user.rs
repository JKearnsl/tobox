use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>
}
