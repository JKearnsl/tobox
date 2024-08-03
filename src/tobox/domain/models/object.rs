use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::r#box::BoxId;

pub type ObjectId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    pub id: ObjectId,
    pub name: Option<String>,
    pub hash: String,
    pub size: u64,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
    pub box_id: BoxId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}
