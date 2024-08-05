use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::models::permission::PermissionTag;
use crate::domain::models::user::UserId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub user_id: UserId,
    pub permissions: Vec<PermissionTag>,
    pub expires_at: DateTime<Utc>
}
