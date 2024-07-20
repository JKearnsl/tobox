use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::common::email_sender::EmailSender;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::config::Extra;
use crate::domain::models::user::{UserId, UserState};
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResultDTO{
    id: UserId,
    username: String,
    email: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct CreateUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub role_gateway: &'a dyn RoleGateway,
    pub email_sender: &'a dyn EmailSender,
    pub user_service: &'a UserService,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
    pub extra: &'a Extra,
}

impl Interactor<CreateUserDTO, CreateUserResultDTO> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserDTO) -> Result<CreateUserResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_create_user(
            &self.id_provider.permissions()
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

        self.validator.validate_email(&data.email).unwrap_or_else(|e| {
            validator_err_map.insert("email".to_string(), e.to_string());
        });

        if let Some(first_name) = &data.first_name {
            self.validator.validate_first_name(first_name).unwrap_or_else(|e| {
                validator_err_map.insert("first_name".to_string(), e.to_string());
            });
        }

        if let Some(last_name) = &data.last_name {
            self.validator.validate_last_name(last_name).unwrap_or_else(|e| {
                validator_err_map.insert("last_name".to_string(), e.to_string());
            });
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let default_role_id = match self.role_gateway.get_default_role().await {
            Some(role) => role.id,
            None => {
                return Err(ApplicationError::Forbidden(
                    ErrorContent::Message(
                        "Сервис на стадии инициализации. Роль по умолчанию не установлена!".to_string()
                    )
                ))
            }
        };

        // Todo: to gather
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(&data.username).await;
        let user_by_email = self.user_gateway.get_user_by_email_not_sensitive(&data.email).await;

        // let (user_by_username, user_by_email) = match tokio::try_join!(
        //     tokio::spawn(async move { self.user_gateway.get_user_by_username_not_sensitive(&data.username).await }),
        //     tokio::spawn(async move { self.user_gateway.get_user_by_email_not_sensitive(&data.email).await })
        // ) {
        //     Ok((user_by_username, user_by_email)) => (user_by_username, user_by_email),
        //     Err(e) => panic!("Error: {:?}", e)
        // };
        
        if user_by_username.is_some() {
            validator_err_map.insert("username".to_string(), "Имя пользователя занято".to_string());
        }


        if user_by_email.is_some() {
            validator_err_map.insert("email".to_string(), "Пользователь с таким Email уже существует".to_string());
        }


        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }


        let hashed_password = self.password_hasher.hash(&data.password).await;
        
        let user = self.user_service.create_user(
            data.username,
            data.email,
            UserState::Inactive,
            hashed_password,
            data.first_name,
            data.last_name,
        );

        self.user_gateway.save_user(&user).await;
        self.role_gateway.link_role_to_user(&default_role_id, &user.id).await;
        
        let context: BTreeMap<String, Value> = {
            let mut context = BTreeMap::new();
            context.insert("username".to_string(), Value::String(user.username.to_string()));
            context.insert("company".to_string(), Value::String(self.extra.company.to_string()));
            context.insert("company_url".to_string(), Value::String(self.extra.company_url.to_string()));
            context
        };
        
        self.email_sender.send_template(
            &user.email,
            "Регистрация на сайте",
            "registration.html",
            Some(context),
            13,
            3600
            
        ).await;

        Ok(CreateUserResultDTO {
            id: user.id,
            username: user.username,
            email: user.email,
            state: user.state,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
