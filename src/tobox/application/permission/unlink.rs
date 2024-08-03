use std::collections::HashMap;

use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::role_gateway::RoleReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::PermissionId;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct UnlinkRolePermissionDTO {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}

pub struct UnlinkRolePermission<'a> {
    pub role_reader: &'a dyn RoleReader,
    pub permission_gateway: &'a dyn PermissionGateway,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<UnlinkRolePermissionDTO, ()> for UnlinkRolePermission<'_> {
    async fn execute(&self, data: UnlinkRolePermissionDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_link_permission(
            self.id_provider.is_auth(),
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        if self.role_reader.get_role(&data.role_id).await.is_none() {
            validator_err_map.insert("role_id".to_string(), "Role not found".to_string());
        }

        if self.permission_gateway.get_permission(&data.permission_id).await.is_none() {
            validator_err_map.insert("permission_id".to_string(), "Permission not found".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        if !self.permission_gateway.is_permission_linked_to_role(&data.role_id, &data.permission_id).await {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Permission is not linked to role".to_string())
                )
            )
        }
        
        self.permission_gateway.unlink_permission_from_role(&data.role_id, &data.permission_id).await;
        
        Ok(())
    }
}
