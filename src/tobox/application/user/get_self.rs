use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::{UserId, UserState};
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
        
        match self.access_service.ensure_can_get_user_self(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(error.to_string())
                )
            )
        };
        
        
        let user = self.user_reader.get_user_by_id(self.id_provider.user_id().unwrap()).await.unwrap();

        Ok(UserSelfResultDTO {
            id: user.id,
            username: user.username,
            created_at: user.created_at
        })
    }
}