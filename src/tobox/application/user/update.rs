use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub id: UserId,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResultDTO{
    id: UserId,
    username: String,
    created_at: DateTime<Utc>
}


pub struct UpdateUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub id_provider: Box<dyn IdProvider>,
    pub password_hasher: &'a dyn Hasher,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<UpdateUserDTO, UpdateUserResultDTO> for UpdateUser<'_> {
    async fn execute(&self, data: UpdateUserDTO) -> Result<UpdateUserResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_user(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
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
        }

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });
        
        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("email".to_string(), e.to_string());
        });

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(&data.username).await;

        if user_by_username.is_some() && user_by_username.unwrap().id != data.id {
            validator_err_map.insert("username".to_string(), "Username taken".to_string());
        }
        
        let user = match self.user_gateway.get_user_by_id(&data.id).await {
            Some(user) => user,
            None => {
                return Err(ApplicationError::NotFound(
                    ErrorContent::Message("User not found".to_string()))
                );
            }
        };
        
        let new_user = self.user_service.update_user(
            user.clone(),
            data.username,
            user.hashed_password
        );

        self.user_gateway.save_user(&new_user).await;
        
        // todo: sync with other services

        Ok(UpdateUserResultDTO {
            id: new_user.id,
            username: new_user.username,
            created_at: new_user.created_at
        })
    }
}