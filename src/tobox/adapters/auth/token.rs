use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::application::common::id_provider::IdProvider;
use crate::domain::models::permission::PermissionTag;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

pub struct IdTokenProvider {
    token: Option<String>,
    user_id: Option<UserId>,
    permissions: Vec<PermissionTag>,
    is_auth: bool
}


impl IdTokenProvider {
    pub fn new(
        token: Option<String>,
        token_processor: &TokenProcessor,
    ) -> Result<Self, String> {
        match token {
            Some(token) => {
                let session = token_processor.get_token_session(&token)?;
                Ok(Self {
                    token: Some(token),
                    user_id: Option::from(session.user_id),
                    permissions: session.permissions,
                    is_auth: true
                })
            }
            None => {
                Ok(Self {
                    token,
                    user_id: None,
                    permissions: PermissionTag::guest_tags(),
                    is_auth: false
                })
            }
        }
    }
}

impl IdProvider for IdTokenProvider {
    fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }
    fn permissions(&self) -> &Vec<PermissionTag> {
        &self.permissions
    }
    fn is_auth(&self) -> &bool {
        &self.is_auth
    }
}


pub struct TokenProcessor {
    data: Arc<RwLock<HashMap<String, Session>>>,
}

impl TokenProcessor {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn set_token_session(&self, session: &Session) -> String {
        let token = session.token.clone();
        let mut data = self.data.write().unwrap();
        data.insert(token.clone(), session.clone());
        token
    }
    
    pub fn get_token_session(&self, token: &str) -> Result<Session, String> {
        let data = self.data.read().unwrap();
        match data.get(token) {
            Some(session) => {
                if session.expires_at < chrono::Utc::now() {
                    Ok(session.clone())
                } else {
                    Err("Token expired".to_string())
                }
            }
            None => Err("Token not found".to_string())
        }
    }
}
