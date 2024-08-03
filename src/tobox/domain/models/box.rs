use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type BoxId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Box {
    pub id: BoxId,
    pub name: String,
    pub created_at: DateTime<Utc>
}
