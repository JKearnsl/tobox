use uuid::Uuid;
use crate::domain::models::permission::{Permission, PermissionTextId};

pub struct PermissionService { }

impl PermissionService {

    pub fn create_permission(
        &self,
        text_id: PermissionTextId,
    ) -> Permission {
        Permission {
            id: Uuid::new_v4(),
            text_id,
        }
    }
}
