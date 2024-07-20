use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{PermissionId, PermissionTextId};
use crate::domain::models::service::ServiceId;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::permission::PermissionService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionDTO {
    pub id: UserId,
    pub title: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize)]
pub struct UpdatePermissionResultDTO{
    id: PermissionId,
    text_id: PermissionTextId,
    
    service_id: ServiceId,
    title: String,
    description: Option<String>,
    
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct UpdatePermission<'a> {
    pub permission_gateway: &'a dyn PermissionGateway,
    pub permission_service: &'a PermissionService,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<UpdatePermissionDTO, UpdatePermissionResultDTO> for UpdatePermission<'_> {
    async fn execute(&self, data: UpdatePermissionDTO) -> Result<UpdatePermissionResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_permission(
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
        self.validator.validate_permission_title(&data.title).unwrap_or_else(|e| {
            validator_err_map.insert("title".to_string(), e.to_string());
        });
        
        if let Some(description) = &data.description {
            self.validator.validate_permission_description(description).unwrap_or_else(|e| {
                validator_err_map.insert("description".to_string(), e.to_string());
            });
        }
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let permission = self.permission_gateway.get_permission_by_id(
            &data.id
        ).await.ok_or(
            ApplicationError::InvalidData(
                ErrorContent::Message("Указанный идентификатор разрешения не найден".to_string())
            )
        )?;
        
        let new_permission = self.permission_service.update_permission(
            permission,
            data.title,
            data.description
        );
        
        self.permission_gateway.save_permission(&new_permission).await;
        
        Ok(
            UpdatePermissionResultDTO {
                id: new_permission.id,
                text_id: new_permission.text_id,
                service_id: new_permission.service_id,
                title: new_permission.title,
                description: new_permission.description,
                created_at: new_permission.created_at,
                updated_at: new_permission.updated_at
            }
        )
    }
}