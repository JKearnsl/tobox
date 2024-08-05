use rand::random;
use crate::domain::models::permission::PermissionTag;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

pub struct SessionService {
    token_ttl: u32,
}

impl SessionService {
    
    /// param token_ttl: Token time to live in seconds
    pub fn new(token_ttl: u32) -> SessionService {
        SessionService {
            token_ttl,
        }
    }
    pub fn create_session(
        &self,
        user_id: UserId,
        permissions: Vec<PermissionTag>,
    ) -> Session {
        Session {
            token: (0..64).map(
                |_| format!("{:02x}", random::<u8>())
            ).collect::<Vec<_>>().join("").to_string(),
            user_id,
            permissions,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(self.token_ttl as i64)
        }
    }
}
