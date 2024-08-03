use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;
use crate::domain::services::role::RoleService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct UpdateRoleDTO {
    pub id: RoleId,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleResultDTO{
    id: RoleId,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct UpdateRole<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub role_service: &'a RoleService,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService,
}

impl Interactor<UpdateRoleDTO, RoleResultDTO> for UpdateRole<'_> {
    async fn execute(&self, data: UpdateRoleDTO) -> Result<RoleResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_role(
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
        self.validator.validate_role_title(&data.title).unwrap_or_else(|e| {
            validator_err_map.insert("title".to_string(), e.to_string());
        });

        if data.description.is_some() {
            self.validator.validate_role_description(&data.description.clone().unwrap()).unwrap_or_else(
                |e| {
                    validator_err_map.insert("description".to_string(), e.to_string());
                }
            );
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let old_role = match self.role_gateway.get_role(&data.id).await {
            Some(role) => role,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Role not found".to_string())
                )
            )
        };

        // Tobox does not allow the creation of roles with the same names. 
        // However, if due to a synchronization error, two roles with the same name appear, 
        // it does allow this. This was done to reduce the number of conflicts and
        // further operations to resolve them.
        self.role_gateway.get_role_by_title_not_sensitive(&data.title).await.ok_or_else(
            || ApplicationError::InvalidData(
                ErrorContent::Map(
                    [("title".to_string(), "Role with this title already exists".to_string())]
                    .iter().cloned().collect()
                )
            )
        )?;
        
        let new_role = match self.role_service.update_role(
            old_role,
            data.title,
            data.description
        ) {
            Ok(role) => role,
            Err(error) => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message(error.to_string())
                )
            )
        };
        
        self.role_gateway.save_role(&new_role).await;
        
        // todo: sync with other services
        
        Ok(RoleResultDTO{
            id: new_role.id,
            title: new_role.title,
            description: new_role.description,
            created_at: new_role.created_at,
            updated_at: new_role.updated_at,
        })
    }
}
