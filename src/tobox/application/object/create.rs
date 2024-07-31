use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::box_gateway::BoxReader;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::file_storage_manager::FileStorageWriter;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::object_gateway::ObjectGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::file_stream::FileStream;
use crate::domain::models::object::ObjectId;
use crate::domain::services::access::AccessService;
use crate::domain::services::object::ObjectService;
use crate::domain::services::validator::ValidatorService;

#[derive(Deserialize)]
pub struct CreateObjectDTO {
    pub box_name: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub file: dyn FileStream,
    pub metadata: HashMap<String, String>
}

#[derive(Debug, Serialize)] 
pub struct CreateObjectResultDTO{
    pub id: ObjectId,
    pub name: String,
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

pub struct CreateObject<'a> {
    pub box_reader: &'a dyn BoxReader,
    pub file_storage_writer: &'a dyn FileStorageWriter,
    pub object_gateway: &'a dyn ObjectGateway,
    pub object_service: &'a ObjectService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateObjectDTO, CreateObjectResultDTO> for CreateObject<'_> {
    async fn execute(&self, data: CreateObjectDTO) -> Result<CreateObjectResultDTO, ApplicationError> {
        
        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        
        self.validator.validate_box_name(&data.box_name).unwrap_or_else(|e| {
            validator_err_map.insert("box_name".to_string(), e.to_string());
        });
        
        if let Some(name) = &data.name {
            self.validator.validate_object_name(name).unwrap_or_else(|e| {
                validator_err_map.insert("name".to_string(), e.to_string());
            });
        }
        
        if let Some(path) = &data.path {
            self.validator.validate_object_path(path).unwrap_or_else(|e| {
                validator_err_map.insert("path".to_string(), e.to_string());
            });
        }
        
        // todo: validate metadata
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        
        let r#box = match self.box_reader.get_box_by_name_not_sensitive(&data.box_name).await {
            Some(r#box) => r#box,
            None => {
                validator_err_map.insert("box_name".to_string(), "Box not found".to_string());
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Map(validator_err_map)
                    )
                )
            }
        };

        match self.access_service.ensure_can_create_object(
            self.id_provider.is_auth(),
            &r#box.id,
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
        
        let object = self.object_service.create_object(
            data.name,
            data.path,
            "".to_string(),
            0,
            "".to_string(),
            data.metadata,
        );

        let file_hash = self.file_storage_writer.save_file(
            object.id,
            None,
            None,
            data.file
        ).await;

        self.object_gateway.save_object(&object).await;
        
        // Todo: sync with other nodes in background

        Ok(CreateObjectResultDTO {
            id: object.id,
            name: object.name,
            path: object.path,
            hash: object.hash,
            size: object.size,
            content_type: object.content_type,
            metadata: object.metadata,
            created_at: object.created_at,
            updated_at: None,
        })
    }
}
