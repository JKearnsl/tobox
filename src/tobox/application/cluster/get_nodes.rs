use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;
use serde_json::Value;

use crate::application::common::email_sender::EmailSender;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::config::Extra;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordDTO {
    pub old_password: String,
    pub new_password: String,
}

pub struct ChangePassword<'a> {
    pub email_sender: &'a dyn EmailSender,
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub validator: &'a ValidatorService,
    pub password_hasher: &'a dyn Hasher,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
    pub extra: &'a Extra,
}

impl Interactor<ChangePasswordDTO, ()> for ChangePassword<'_> {
    async fn execute(&self, data: ChangePasswordDTO) -> Result<(), ApplicationError> {

        match self.access_service.ensure_can_update_user_self(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(error.to_string())
                )
            )
        };

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_password(&data.new_password).unwrap_or_else(|e| {
            validator_err_map.insert("new_password".to_string(), e.to_string());
        });
        
        if data.old_password == data.new_password {
            validator_err_map.insert(
                "new_password".to_string(), "Новый пароль совпадает с текущим".to_string()
            );
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }

        let user = self.user_gateway.get_user_by_id(self.id_provider.user_id().unwrap()).await.ok_or(
            ApplicationError::NotFound(
                ErrorContent::Message("Пользователь не найден".to_string())
            )
        )?;

        let is_valid = self.password_hasher.verify(
            &data.old_password, 
            &user.hashed_password
        ).await;
        
        if !is_valid {
            return Err(ApplicationError::InvalidData(
                ErrorContent::Map(
                    [("old_password".to_string(), "Неверный пароль".to_string())]
                        .iter().cloned().collect()
                )
            ))
        }
        
        let new_user = self.user_service.update_user(
            user.clone(),
            user.username.clone(),
            user.email.clone(),
            user.state,
            user.first_name,
            user.last_name,
            self.password_hasher.hash(&data.new_password).await,
        );
        
        self.user_gateway.save_user(&new_user).await;
        
        let context: BTreeMap<String, Value> = {
            let mut context = BTreeMap::new();
            context.insert("username".to_string(), Value::String(user.username.clone()));
            context.insert("ip".to_string(), Value::String(self.id_provider.ip().to_string()));
            context.insert("change_time".to_string(), Value::String({
                let now = chrono::Utc::now();
                now.format("%d/%m/%Y %H:%M %Z").to_string()
            }));
            context.insert("email".to_string(), Value::String(user.email.clone()));
            context.insert("company".to_string(), Value::String(self.extra.company.clone()));
            context.insert("company_url".to_string(), Value::String(self.extra.company_url.clone()));
            context.insert("reset_password_url".to_string(), Value::String(self.extra.reset_password_url.clone()));
            context
        };
        
        self.email_sender.send_template(
            &user.email,
            "Изменение пароля",
            "successfully_reset_password.html",
            Some(context),
            13,
            3600
        ).await;
        
        Ok(())
    }
}
