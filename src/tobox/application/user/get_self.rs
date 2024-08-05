use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Serialize)]
pub struct UserSelfResultDTO{
    id: UserId,
    username: String,
    created_at: DateTime<Utc>
}


pub struct GetUserSelf<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<(), UserSelfResultDTO> for GetUserSelf<'_> {
    async fn execute(&self, _data: ()) -> Result<UserSelfResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_user(
            self.id_provider.is_auth(),
            &self.id_provider.permissions()
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
        
        let user = self.user_reader.get_user(
            self.id_provider.user_id().unwrap()
        ).await.unwrap();

        Ok(UserSelfResultDTO {
            id: user.id,
            username: user.username,
            created_at: user.created_at
        })
    }
}