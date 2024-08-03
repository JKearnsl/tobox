use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionReader;
use crate::application::common::role_gateway::RoleReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{PermissionId, PermissionTag};
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;


#[derive(Debug, Serialize)]
pub struct PermissionItem {
    pub id: PermissionId,
    pub tag: PermissionTag,
}


#[derive(Serialize)]
pub struct GetDefaultRoleResult {
    id: RoleId,
    title: String,
    description: Option<String>,
    permissions: Vec<PermissionItem>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub struct GetDefaultRole<'a> {
    pub role_reader: &'a dyn RoleReader,
    pub permission_reader: &'a dyn PermissionReader,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), GetDefaultRoleResult> for GetDefaultRole<'_> {
    async fn execute(&self, _data: ()) -> Result<GetDefaultRoleResult, ApplicationError> {
        
        match self.access_service.ensure_can_get_role(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return match error {
                DomainError::AccessDenied => Err(
                    ApplicationError::Forbidden(
                        ErrorContent::Message(error.to_string())
                    )
                ),
                DomainError::AuthorizationRequired => Err(
                    ApplicationError::Unauthorized(
                        ErrorContent::Message(error.to_string())
                    )
                )
            }
        };
        
        
        let role = self.role_reader.get_default_role().await.ok_or(
            ApplicationError::NotFound(
                ErrorContent::Message("The default role has not been set yet".to_string())
            )
        )?;
        
        let permissions = self.permission_reader.get_role_permissions(&role.id).await;
        
        Ok(
            GetDefaultRoleResult {
                id: role.id,
                title: role.title,
                description: role.description,
                permissions: permissions.iter().map(|permission| PermissionItem {
                    id: permission.id.clone(),
                    tag: permission.tag.clone()
                }).collect(),
                created_at: role.created_at,
                updated_at: role.updated_at
            }
        )
    }
}
