use rand::random;
use uuid::Uuid;

use crate::domain::models::session::{Session, SessionToken, SessionTokenHash};

pub struct SessionService {
    session_expire: u32,
}

impl SessionService {

    pub fn new(session_expire: u32) -> SessionService {
        SessionService {
            session_expire,
        }
    }
    
    pub fn is_session_expired(&self, session: &Session) -> bool {
        let session_age = chrono::Utc::now() - session.updated_at.unwrap_or(session.created_at);
        session_age > chrono::Duration::seconds(self.session_expire as i64)
    }
    
    pub fn create_session_token(&self) -> SessionToken {
        (0..64).map(|_| format!("{:02x}", random::<u8>())).collect::<Vec<_>>().join("")
    }

    pub fn create_session(
        &self,
        token_hash: SessionTokenHash,
        user_id: Uuid,
        ip: String,
        client: String,
        os: String,
        device: String,
    ) -> Session {
        Session {
            id: Uuid::new_v4(),
            token_hash,
            user_id,
            ip,
            client,
            os,
            device,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }

    pub fn verify_session(
        &self,
        session: &Session,
        client: &str,
        os: &str,
        device: &str,
    ) -> bool {
        session.client == client && session.os == os && session.device == device
    }

    pub fn update_session(
        &self,
        session: Session,
        new_ip: String,
    ) -> Session {
        Session {
            ip: new_ip,
            updated_at: Some(chrono::Utc::now()),
            ..session
        }
    }
}
