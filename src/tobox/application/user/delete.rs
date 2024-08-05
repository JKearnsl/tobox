use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct DeleteUserDTO {
    pub id: UserId,
}


pub struct DeleteUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<DeleteUserDTO, ()> for DeleteUser<'_> {
    async fn execute(&self, data: DeleteUserDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_delete_user(
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
        }
        
        match self.user_gateway.get_user(&data.id).await {
            Some(user) => user,
            None => {
                return Err(
                    ApplicationError::NotFound(ErrorContent::from("User not found"))
                );
            }
        };
        
        self.user_gateway.remove_user(&data.id).await;
        
        // todo: sync with other services

        Ok(())
    }
}