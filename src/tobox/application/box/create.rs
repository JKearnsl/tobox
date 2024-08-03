use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::box_gateway::BoxGateway;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::exceptions::DomainError;
use crate::domain::models::r#box::BoxId;
use crate::domain::services::access::AccessService;
use crate::domain::services::r#box::BoxService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Serialize)]
pub struct CreateBoxResultDTO{
    id: BoxId,
    created_at: DateTime<Utc>
}

pub struct CreateBox<'a> {
    pub box_gateway: &'a dyn BoxGateway,
    pub box_service: &'a BoxService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), CreateBoxResultDTO> for CreateBox<'_> {
    async fn execute(&self, _data: ()) -> Result<CreateBoxResultDTO, ApplicationError> {
        
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
        
        let r#box = self.box_service.create_box();
        
        self.box_gateway.save_box(&r#box).await;

        Ok(CreateBoxResultDTO {
            id: r#box.id,
            created_at: r#box.created_at
        })
    }
}
