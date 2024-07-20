use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type RoleId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}