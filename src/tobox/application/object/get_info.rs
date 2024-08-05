use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::file_storage_manager::FileStorageReader;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::object_gateway::ObjectReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::object::ObjectId;
use crate::domain::models::r#box::BoxId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetObjectInfoDTO {
    pub id: ObjectId
}

#[derive(Debug, Deserialize)]
pub struct GetObjectInfoResultDTO {
    pub id: ObjectId,
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
    pub box_id: BoxId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

pub struct GetObjectInfo<'a> {  
    pub object_reader: &'a dyn ObjectReader,
    pub file_storage_reader: &'a dyn FileStorageReader,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>
}

impl Interactor<GetObjectInfoDTO, GetObjectInfoResultDTO> for GetObjectInfo<'_> {
    async fn execute(&self, data: GetObjectInfoDTO) -> Result<GetObjectInfoResultDTO, ApplicationError> {

        let object = self.object_reader.get_object(&data.id).await.ok_or(
            ApplicationError::NotFound(ErrorContent::from("Object not found"))
        )?;

        match self.access_service.ensure_can_get_object(
            self.id_provider.is_auth(),
            &object.box_id,
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
        
        Ok(GetObjectInfoResultDTO {
            id: object.id.clone(),
            name: object.name.unwrap_or(object.id.clone()),
            hash: object.hash,
            size: object.size,
            content_type: object.content_type,
            metadata: object.metadata,
            box_id: object.box_id,
            created_at: object.created_at,
            updated_at: object.updated_at
        })
    }
}
