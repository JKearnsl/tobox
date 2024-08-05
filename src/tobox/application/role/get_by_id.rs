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
pub struct GetRoleByIdDTO {
    pub id: RoleId,
}

#[derive(Debug, Serialize)]
pub struct RoleByIdResultDTO{
    id: RoleId,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct GetRoleById<'a> {
    pub role_reader: &'a dyn RoleReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetRoleByIdDTO, RoleByIdResultDTO> for GetRoleById<'_> {
    async fn execute(&self, data: GetRoleByIdDTO) -> Result<RoleByIdResultDTO, ApplicationError> {
        
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
        
        let role = match self.role_reader.get_role(&data.id).await {
            Some(role) => role,
            None => return Err(
                ApplicationError::InvalidData(ErrorContent::from("Role not found"))
            )
        };
        
        Ok(RoleByIdResultDTO{
            id: role.id,
            title: role.title,
            description: role.description,
            created_at: role.created_at,
            updated_at: role.updated_at,
        })
    }
}
