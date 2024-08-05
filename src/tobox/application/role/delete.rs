use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct DeleteRoleDTO {
    id: RoleId,
}

pub struct DeleteRole<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<DeleteRoleDTO, ()> for DeleteRole<'_> {
    async fn execute(&self, data: DeleteRoleDTO) -> Result<(), ApplicationError> {

        match self.access_service.ensure_can_delete_role(
            self.id_provider.is_auth(),
            &self.id_provider.permissions()
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
        
        self.role_gateway.get_role(&data.id).await.ok_or_else(|| {
            ApplicationError::NotFound(ErrorContent::from("Role not found"))
        })?;

        self.role_gateway.remove_role(&data.id).await;
        
        // todo: sync with other services
        
        Ok(())
    }
}
