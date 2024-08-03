use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdDTO {
    pub id: UserId,
}

#[derive(Debug, Serialize)]
pub struct UserByIdResultDTO{
    id: UserId,
    username: String,
    created_at: DateTime<Utc>
}


pub struct GetUserById<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetUserByIdDTO, UserByIdResultDTO> for GetUserById<'_> {
    async fn execute(&self, data: GetUserByIdDTO) -> Result<UserByIdResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_user(
            self.id_provider.is_auth(),
            self.id_provider.user_id(),
            &data.id,
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => match error {
                DomainError::AccessDenied => return Err(
                    ApplicationError::Forbidden(
                        ErrorContent::Message(error.to_string())
                    )
                ),
                DomainError::AuthorizationRequired => return Err(
                    ApplicationError::Unauthorized(
                        ErrorContent::Message(error.to_string())
                    )
                )
            }
        };
        
        let user = match self.user_reader.get_user(&data.id).await {
            Some(u) => u,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("User not found".to_string())
                )
            ),
        };

        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        })
    }
}