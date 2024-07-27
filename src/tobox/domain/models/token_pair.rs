use serde::{Deserialize, Serialize};

use crate::domain::models::user::UserId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub public: String,
    pub enc_private: String,
    pub user_id: UserId
}
