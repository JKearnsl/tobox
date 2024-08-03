use crate::domain::id_generator::generate_id;
use crate::domain::models::permission::{Permission, PermissionTextId};

pub struct PermissionService { }

impl PermissionService {

    pub fn create_permission(
        &self,
        text_id: PermissionTextId,
    ) -> Permission {
        Permission {
            id: generate_id(16),
            text_id,
        }
    }
}
