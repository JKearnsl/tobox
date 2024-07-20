use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::role_gateway::RoleGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{PermissionId, PermissionTextId};
use crate::domain::models::role::RoleId;
use crate::domain::models::service::ServiceId;
use crate::domain::services::access::AccessService;
use crate::domain::services::role::RoleService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateRoleDTO {
    pub title: String,
    pub description: Option<String>,
    pub permissions: Vec<PermissionId>
}

#[derive(Debug, Serialize)]
pub struct PermissionItem {
    pub id: PermissionId,
    pub text_id: PermissionTextId,
    pub service_id: ServiceId,
    pub title: String,
    pub description: Option<String>
}


#[derive(Debug, Serialize)]
pub struct CreateRoleResultDTO{
    id: RoleId,
    title: String,
    description: Option<String>,
    permissions: Vec<PermissionItem>
}

pub struct CreateRole<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub permission_gateway: &'a dyn PermissionGateway,
    pub role_service: &'a RoleService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateRoleDTO, CreateRoleResultDTO> for CreateRole<'_> {
    async fn execute(&self, data: CreateRoleDTO) -> Result<CreateRoleResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_create_role(
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

        let role_by_title = self.role_gateway.get_role_by_title_not_sensitive(&data.title).await;
        
        if role_by_title.is_some() {
            validator_err_map.insert("title".to_string(), "Роль с таким названием уже существует".to_string());
        }
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let permissions = match self.permission_gateway.get_permissions_by_ids(
            &data.permissions
        ).await {
            Some(permissions) => permissions,
            None => {
                validator_err_map.insert(
                    "permission".to_string(), "Не все указанные права были найдены".to_string()
                );
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Map(validator_err_map)
                    )
                )
            }
        };
        
        let role = self.role_service.create_role(
            data.title,
            data.description,
        );
        
        self.role_gateway.save_role(&role).await;
        
        if !data.permissions.is_empty() {
            self.permission_gateway.link_permissions_to_role(
                &role.id,
                &data.permissions
            ).await;
        }
        
        Ok(CreateRoleResultDTO {
            id: role.id,
            title: role.title,
            description: role.description,
            permissions: permissions.iter().map(|permission| {
                PermissionItem {
                    id: permission.id,
                    text_id: permission.text_id.clone(),
                    service_id: permission.service_id.clone(),
                    title: permission.title.clone(),
                    description: permission.description.clone()
                }
            }).collect()
        })
    }
}
