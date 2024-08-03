use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleReader;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetUserRolesDTO {
    pub user_id: RoleId,
}

#[derive(Debug, Serialize)]
pub struct RoleItemResult{
    id: RoleId,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub type UserRolesResultDTO = Vec<RoleItemResult>;


pub struct GetUserRoles<'a> {
    pub role_reader: &'a dyn RoleReader,
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetUserRolesDTO, UserRolesResultDTO> for GetUserRoles<'_> {
    async fn execute(&self, data: GetUserRolesDTO) -> Result<UserRolesResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_user_roles(
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
        
        match self.user_reader.get_user_by_id(&data.user_id).await {
            Some(_) => (),
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("User not found".to_string())
                )
            )
        };
        
        let roles = self.role_reader.get_user_roles(&data.user_id).await;
        
        Ok(roles.into_iter().map(|role| RoleItemResult {
            id: role.id,
            title: role.title,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }).collect())
    }
}