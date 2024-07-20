use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::permission::{Permission, PermissionTextId};
use crate::domain::models::service::ServiceId;

pub struct PermissionService {}

impl PermissionService {
    pub fn create_permission(
        &self,
        text_id: PermissionTextId,
        service_id: ServiceId,
        title: String,
        description: Option<String>,
    ) -> Permission {
        Permission {
            id: Uuid::new_v4(),
            text_id,
            service_id,
            title,
            description,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
    
    pub fn update_permission(
        &self,
        permission: Permission,
        new_title: String,
        new_description: Option<String>,
    ) -> Permission {
        Permission {
            title: new_title,
            description: new_description,
            updated_at: Some(Utc::now()),
            ..permission
        }
    }
}
