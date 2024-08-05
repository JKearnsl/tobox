use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionReader;
use crate::application::common::session_gateway::SessionWriter;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::session::SessionService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResultDTO{
    token: String,
    user_id: UserId,
    permissions: Vec<String>,
    expires_at: DateTime<Utc>
}

pub struct CreateSession<'a> {
    pub session_writer: &'a dyn SessionWriter,
    pub session_service: &'a SessionService,
    pub user_reader: &'a dyn UserReader,
    pub permission_reader: &'a dyn PermissionReader,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateSessionDTO, CreateSessionResultDTO> for CreateSession<'_> {
    async fn execute(
        &self, 
        data: CreateSessionDTO
    ) -> Result<CreateSessionResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_create_session(
            self.id_provider.is_auth(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(e) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(e.to_string())
                )
            )
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
                ApplicationError::InvalidData(
                    ErrorContent::Message("Invalid username and password pair")
                )
            )
        }
        
        let user = match self.user_reader.get_user_by_username_not_sensitive(&data.username).await {
            Some(user) => user,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Invalid username and password pair")
                )
            )
        };
        
        if !self.password_hasher.verify(data.password, user.hashed_password).await {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Invalid username and password pair")
                )
            )
        }
        
        let permissions = self.permission_reader.get_user_permissions(
            &user.id
        ).await;
        
        let session = self.session_service.create_session(
            user.id.clone(),
            permissions.iter().map(|p| p.tag).collect()
        );
        
        self.session_writer.write_session(session.clone()).await;

        Ok(CreateSessionResultDTO {
            token: session.token,
            user_id: user.id,
            permissions: permissions.iter().map(|p| p.tag).collect(),
            expires_at: session.expires_at
        })
    }
}
