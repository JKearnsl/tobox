use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type UserId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>
}
