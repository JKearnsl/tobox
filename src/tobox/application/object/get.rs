use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::file_storage_manager::FileStorageReader;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::object_gateway::ObjectReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::file_stream::FileStream;
use crate::domain::models::object::ObjectId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetObjectDTO {
    pub id: ObjectId
}

pub struct GetObject<'a> {
    pub file_storage_reader: &'a dyn FileStorageReader,
    pub object_reader: &'a dyn ObjectReader,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>
}

impl Interactor<GetObjectDTO, Box<dyn FileStream>> for GetObject<'_> {
    async fn execute(&self, data: GetObjectDTO) -> Result<Box<dyn FileStream>, ApplicationError> {
        
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

        Ok(self.file_storage_reader.read_file(&object.hash).await)
    }
}
