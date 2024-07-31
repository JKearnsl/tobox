use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::box_gateway::BoxGateway;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::exceptions::DomainError;
use crate::domain::models::r#box::BoxId;
use crate::domain::services::access::AccessService;
use crate::domain::services::r#box::BoxService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateBoxDTO {
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct CreateBoxResultDTO{
    id: BoxId,
    name: String,
    created_at: DateTime<Utc>
}

pub struct CreateBox<'a> {
    pub box_gateway: &'a dyn BoxGateway,
    pub box_service: &'a BoxService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateBoxDTO, CreateBoxResultDTO> for CreateBox<'_> {
    async fn execute(&self, data: CreateBoxDTO) -> Result<CreateBoxResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_create_box(
            self.id_provider.is_auth(),
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
        self.validator.validate_username(&data.name).unwrap_or_else(|e| {
            validator_err_map.insert("name".to_string(), e.to_string());
        });
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        match self.box_gateway.get_box_by_name_not_sensitive(&data.name).await {
            Some(_) => {
                validator_err_map.insert("name".to_string(), "Name is already taken".to_string());
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Map(validator_err_map)
                    )
                )
            },
            None => ()
        }
        
        let r#box = self.box_service.create_box(data.name);
        
        self.box_gateway.save_box(&r#box).await;

        Ok(CreateBoxResultDTO {
            id: r#box.id,
            name: r#box.name,
            created_at: r#box.created_at
        })
    }
}
