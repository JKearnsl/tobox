use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct GetUserRangeDTO {
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct UserItemResult{
    id: UserId,
    username: String,
    created_at: DateTime<Utc>
}

pub type GetUserRangeResultDTO = Vec<UserItemResult>;


pub struct GetUserRange<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService,
}

impl Interactor<GetUserRangeDTO, GetUserRangeResultDTO> for GetUserRange<'_> {
    async fn execute(&self, data: GetUserRangeDTO) -> Result<GetUserRangeResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_user_range(
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
        
        let users = self.user_reader.get_users_list(
            &data.per_page,
            &(data.page * data.per_page)
        ).await;
        
        Ok(
            users.into_iter().map(|user| UserItemResult {
                id: user.id,
                username: user.username,
                created_at: user.created_at
            }).collect()
        )
    }
}