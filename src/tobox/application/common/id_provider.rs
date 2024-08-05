use crate::domain::models::permission::PermissionTag;
use crate::domain::models::user::UserId;

pub trait IdProvider {
    fn token(&self) -> Option<&String>;
    fn user_id(&self) -> Option<&UserId>;
    fn permissions(&self) -> &Vec<PermissionTag>;
    fn is_auth(&self) -> &bool;
}
