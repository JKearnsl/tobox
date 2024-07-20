use uuid::Uuid;

use crate::domain::exceptions::DomainError;
use crate::domain::models::role::Role;

pub struct RoleService { }

impl RoleService {

    pub fn create_role(
        &self,
        title: String,
        description: Option<String>,
    ) -> Role {
        Role {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: Default::default(),
            updated_at: None,
        }
    }

    pub fn update_role(
        &self,
        role: Role,
        new_title: String,
        new_description: Option<String>,
    ) -> Result<Role, DomainError> {
        Ok(Role {
            title: new_title,
            description: new_description,
            updated_at: Some(chrono::Utc::now()),
            ..role
        })
    }
}
