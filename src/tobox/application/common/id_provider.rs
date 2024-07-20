use crate::domain::models::session::SessionId;
use crate::domain::models::user::{UserId, UserState};

pub trait IdProvider: Send + Sync {
    fn session_id(&self) -> Option<&SessionId>;
    fn user_id(&self) -> Option<&UserId>;
    fn user_state(&self) -> Option<&UserState>;
    fn permissions(&self) -> &Vec<String>;
    fn client(&self) -> &str;
    fn os(&self) -> &str;
    fn device(&self) -> &str;
    fn ip(&self) -> &str;
    fn is_auth(&self) -> &bool;
}
