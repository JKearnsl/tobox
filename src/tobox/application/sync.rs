use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::application::common::hasher::Hasher;
use crate::application::common::init_state_gateway::InitStateGateway;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::service_gateway::ServiceGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::application::service::sync::{ServiceSync, ServiceSyncDTO};
use crate::domain::models::permission::{PermissionId, PermissionTextId};
use crate::domain::models::service::ServiceTextId;
use crate::domain::models::ums_permission::UMSPermission;
use crate::domain::models::user::UserState;
use crate::domain::services::external::ExternalService;
use crate::domain::services::permission::PermissionService;
use crate::domain::services::role::RoleService;
use crate::domain::services::user::UserService;

pub async fn service_permissions(
    service_gateway: &dyn ServiceGateway,
    permission_gateway: &dyn PermissionGateway,
    permission_service: &PermissionService,
    external_service: &ExternalService
) {
    let permission_text_ids = UMSPermission::iter().map(|permission| {
        permission.to_string()
    }).collect::<Vec<PermissionTextId>>();
    
    let executor = ServiceSync { 
        service_gateway, 
        permission_gateway,
        permission_service,
        external_service
    };

    executor.execute(
        ServiceSyncDTO {
            service_text_id,
            permission_text_ids
        }
    ).await.map_err(|e| {
        log::error!("Service permission sync error: {}", e);
    }).ok();
}
