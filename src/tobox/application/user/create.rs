use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResultDTO{
    id: UserId,
    username: String,
    created_at: DateTime<Utc>
}

pub struct CreateUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub role_gateway: &'a dyn RoleGateway,
    pub user_service: &'a UserService,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateUserDTO, CreateUserResultDTO> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserDTO) -> Result<CreateUserResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_create_user(
            self.id_provider.is_auth(),
            self.id_provider.permissions()
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("password".to_string(), e.to_string());
        });
        

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(ErrorContent::from(validator_err_map))
            )
        }
        
        let default_role_id = match self.role_gateway.get_default_role().await {
            Some(role) => role.id,
            None => {
                return Err(ApplicationError::Forbidden(
                    ErrorContent::from("The default role is not set!")
                ))
            }
        };
        
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(
            &data.username
        ).await;
        
        if user_by_username.is_some() {
            validator_err_map.insert("username".to_string(), "Username taken".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(ErrorContent::from(validator_err_map))
            )
        }
        
        let hashed_password = self.password_hasher.hash(&data.password).await;
        
        let user = self.user_service.create_user(
            data.username,
            hashed_password,
        );

        self.user_gateway.save_user(&user).await;
        self.role_gateway.link_role_to_user(&default_role_id, &user.id).await;
        
        // todo: sync with other services

        Ok(CreateUserResultDTO {
            id: user.id,
            username: user.username,
            created_at: user.created_at
        })
    }
}
