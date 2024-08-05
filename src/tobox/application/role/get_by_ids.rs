use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetRolesByIdsDTO {
    pub ids: Vec<RoleId>,
}


#[derive(Debug, Serialize)]
pub struct RoleItemResult{
    id: RoleId,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub type RolesByIdsResultDTO = Vec<RoleItemResult>;


pub struct GetRolesByIds<'a> {
    pub role_gateway: &'a dyn RoleReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetRolesByIdsDTO, RolesByIdsResultDTO> for GetRolesByIds<'_> {
    async fn execute(&self, data: GetRolesByIdsDTO) -> Result<RolesByIdsResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_role(
            self.id_provider.is_auth(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return match error {
                DomainError::AccessDenied => Err(
                    ApplicationError::Forbidden(ErrorContent::from(error))
                ),
                DomainError::AuthorizationRequired => Err(
                    ApplicationError::Unauthorized(ErrorContent::from(error))
                )
            }
        };
        
        let roles = match self.role_gateway.get_roles(&data.ids).await {
            Some(roles) => roles,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::from("Not all of the specified roles have been found")
                )
            )
        };
        
        Ok(roles.into_iter().map(|role| RoleItemResult{
            id: role.id,
            title: role.title,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at
        }).collect())
    }
}
