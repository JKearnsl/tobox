use serde::Deserialize;

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
pub struct DeleteBoxDTO {
    pub id: BoxId
}

pub struct DeleteBox<'a> {
    pub box_gateway: &'a dyn BoxGateway,
    pub box_service: &'a BoxService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<DeleteBoxDTO, ()> for DeleteBox<'_> {
    async fn execute(&self, data: DeleteBoxDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_delete_box(
            self.id_provider.is_auth(),
            &data.id,
            self.id_provider.permissions()
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
        
        match self.box_gateway.get_box_by_id(&data.id).await {
            Some(_) => (),
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Box not found".to_string())
                )
            )
        }
        
        self.box_gateway.remove_box(&data.id).await;

        Ok(())
    }
}
