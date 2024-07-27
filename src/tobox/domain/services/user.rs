use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::user::User;

pub struct UserService { }

impl UserService {

    pub fn create_user(
        &self,
        username: String,
        hashed_password: String,
    ) -> User {
        User {
            id: Uuid::new_v4(),
            username,
            hashed_password,
            created_at: Utc::now(),
        }
    }

    pub fn update_user(
        &self,
        user: User,
        new_username: String,
        new_hashed_password: String,
    ) -> User {
        User {
            username: new_username,
            hashed_password: new_hashed_password,
            ..user
        }
    }
}
