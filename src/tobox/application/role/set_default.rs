use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

pub struct SetDefaultRole<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<RoleId, ()> for SetDefaultRole<'_> {
    async fn execute(&self, data: RoleId) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_set_default_role(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
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
        
        
        self.role_gateway.get_role(&data).await.ok_or(
            ApplicationError::InvalidData(
                ErrorContent::Message("Role not found".to_string())
            )
        )?;
        
        self.role_gateway.set_default_role(&data).await;

        // todo: sync with other services
        
        Ok(())
    }
}
