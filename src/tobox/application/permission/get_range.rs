use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{PermissionId, PermissionTextId};
use crate::domain::models::service::ServiceId;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct GetPermissionRangeDTO {
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct PermissionItemResult{
    id: PermissionId,
    text_id: PermissionTextId,
    
    service_id: ServiceId,
    title: String,
    description: Option<String>,
    
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub type GetPermissionRangeResultDTO = Vec<PermissionItemResult>;


pub struct GetPermissionRange<'a> {
    pub permission_reader: &'a dyn PermissionReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService,
}

impl Interactor<GetPermissionRangeDTO, GetPermissionRangeResultDTO> for GetPermissionRange<'_> {
    async fn execute(&self, data: GetPermissionRangeDTO) -> Result<GetPermissionRangeResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_permissions(
            &self.id_provider.permissions()
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
        self.validator.validate_page(&data.page).unwrap_or_else(|e| {
            validator_err_map.insert("page".to_string(), e.to_string());
        });
        
        self.validator.validate_per_page(&data.per_page).unwrap_or_else(|e| {
            validator_err_map.insert("per_page".to_string(), e.to_string());
        });
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let permissions = self.permission_reader.get_permissions_list(
            &data.per_page,
            &(data.page * data.per_page)
        ).await;
        
        Ok(
            permissions.into_iter().map(|u| PermissionItemResult {
                id: u.id,
                text_id: u.text_id,
                service_id: u.service_id,
                title: u.title,
                description: u.description,
                created_at: u.created_at,
                updated_at: u.updated_at
            }).collect()
        )
    }
}