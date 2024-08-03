use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::box_gateway::BoxReader;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::exceptions::DomainError;
use crate::domain::models::r#box::BoxId;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct GetBoxListDTO {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct BoxItem {
    pub id: BoxId,
    pub created_at: DateTime<Utc>,
}

pub type GetBoxListResultDTO = Vec<BoxItem>;

pub struct GetBoxList<'a> {
    pub box_reader: &'a dyn BoxReader,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<GetBoxListDTO, GetBoxListResultDTO> for GetBoxList<'_> {
    async fn execute(&self, data: GetBoxListDTO) -> Result<GetBoxListResultDTO, ApplicationError> {

        match self.access_service.ensure_can_get_box(
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
        
        let boxes = if data.page.is_some() && data.per_page.is_none() {
            validator_err_map.insert("per_page".to_string(), "is required".to_string());
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        } else if data.page.is_none() && data.per_page.is_some() {
            validator_err_map.insert("page".to_string(), "is required".to_string());
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        } else if data.page.is_some() && data.per_page.is_some() {
            self.validator.validate_page(&data.page.unwrap()).unwrap_or_else(|e| {
                validator_err_map.insert("page".to_string(), e.to_string());
            });
            self.validator.validate_per_page(&data.per_page.unwrap()).unwrap_or_else(|e| {
                validator_err_map.insert("per_page".to_string(), e.to_string());
            });
            if !validator_err_map.is_empty() {
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Map(validator_err_map)
                    )
                )
            }
            self.box_reader.get_boxes_paginated(data.page.unwrap(), data.per_page.unwrap()).await
        } else {
            self.box_reader.get_boxes().await
        };
        
        Ok(boxes.into_iter().map(|b| BoxItem {
            id: b.id,
            created_at: b.created_at,
        }).collect())
    }
}
