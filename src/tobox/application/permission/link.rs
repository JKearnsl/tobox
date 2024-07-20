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
pub struct LinkRolePermissionDTO {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}

pub struct LinkRolePermission<'a> {
    pub role_reader: &'a dyn RoleReader,
    pub permission_gateway: &'a dyn PermissionGateway,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<LinkRolePermissionDTO, ()> for LinkRolePermission<'_> {
    async fn execute(&self, data: LinkRolePermissionDTO) -> Result<(), ApplicationError> {
        
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
            validator_err_map.insert("role_id".to_string(), "Роль не найдена".to_string());
        }

        if self.permission_gateway.get_permission_by_id(&data.permission_id).await.is_none() {
            validator_err_map.insert("permission_id".to_string(), "Разрешение не найдено".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        if self.permission_gateway.is_permission_linked_to_role(&data.role_id, &data.permission_id).await {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Разрешение уже привязано к данной роли".to_string())
                )
            )
        }
        
        self.permission_gateway.link_permission_to_role(&data.role_id, &data.permission_id).await;
        
        Ok(())
    }
}
