use std::collections::HashMap;

use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct UnlinkRoleUserDTO {
    pub role_id: RoleId,
    pub user_id: UserId,
}

pub struct UnlinkRoleUser<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<UnlinkRoleUserDTO, ()> for UnlinkRoleUser<'_> {
    async fn execute(&self, data: UnlinkRoleUserDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_link_role_user(
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        if self.role_gateway.get_role(&data.role_id).await.is_none() {
            validator_err_map.insert("role_id".to_string(), "Role not found".to_string());
        }

        if self.user_reader.get_user_by_id(&data.user_id).await.is_none() {
            validator_err_map.insert("user_id".to_string(), "User not found".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        if !self.role_gateway.is_role_linked_to_user(&data.role_id, &data.user_id).await {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Role is not linked to user".to_string())
                )
            )
        }
        
        self.role_gateway.unlink_role_from_user(&data.role_id, &data.user_id).await;

        // todo: sync with other services
        
        Ok(())
    }
}
