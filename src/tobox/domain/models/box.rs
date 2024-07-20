use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;


pub type BoxId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Box {
    pub id: BoxId,
    pub name: String,
    pub created_at: DateTime<Utc>
}