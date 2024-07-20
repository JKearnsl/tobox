use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionReader;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{PermissionId, PermissionTextId};
use crate::domain::models::service::ServiceId;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;

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

pub type GetUserPermissionsResultDTO = Vec<PermissionItemResult>;


pub struct GetUserPermissions<'a> {
    pub permission_reader: &'a dyn PermissionReader,
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<UserId, GetUserPermissionsResultDTO> for GetUserPermissions<'_> {
    async fn execute(&self, data: UserId) -> Result<GetUserPermissionsResultDTO, ApplicationError> {
        
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
        
        self.user_reader.get_user_by_id(
            &data
        ).await.ok_or(
            ApplicationError::InvalidData(
                ErrorContent::Message("Пользователь не найден".to_string())
            )
        )?;
        
        let permissions = self.permission_reader.get_user_permissions(
            &data
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