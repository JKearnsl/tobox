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
use crate::domain::models::permission::Permission;
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
    let permission_text_ids = Permission::iter().map(|permission| {
        permission.to_string()
    }).collect::<Vec<String>>();
    
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

pub async fn control_account(
    role_gateway: &dyn RoleGateway,
    role_service: &RoleService,
    permission_gateway: &dyn PermissionGateway,
    user_gateway: &dyn UserGateway,
    user_service: &UserService,
    password_hasher: &dyn Hasher,
    init_state_gateway: &dyn InitStateGateway,
) {
    match init_state_gateway.get_state().await {
        Some(_) => return,
        None => {
            let role = role_service.create_role(
                "Control".to_string(),
                Some("Роль администратора системы".to_string()),
            );
            
            let permission_text_ids = Permission.iter().map(|permission| {
                permission.to_string()
            }).collect::<Vec<PermissionTextId>>();
            
            let permissions = match permission_gateway.get_permissions_by_text_ids(
                &permission_text_ids
            ).await {
                Some(permissions) => permissions,
                None => panic!("Permissions not found in control_account init")
            };
            role_gateway.save_role(&role).await;
            
            permission_gateway.link_permissions_to_role(
                &role.id,
                &permissions.iter().map(
                    |permission| permission.id.clone()
                ).collect::<Vec<PermissionId>>()
            ).await;
            
            // Create control user
            
            let password: String = {
                let mut rng = rand::thread_rng();
                
                let numeric: String = (0..2)
                    .map(|_| rng.gen_range('0'..'9'))
                    .collect();
                
                let alphabetic: String = (0..2)
                    .map(|_| rng.gen_range('A'..'Z'))
                    .collect();

                let alphanumeric: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();

                rng = rand::thread_rng();
                let mut password: Vec<char> = format!(
                    "{}{}{}", numeric, alphabetic, alphanumeric
                ).chars().collect();
                password.shuffle(&mut rng);

                password.into_iter().collect()
            };
            
            let password_hash = password_hasher.hash(password.as_str()).await;
            
            let user = user_service.create_user(
                "control".to_string(),
                "control@milkhunters.ru".to_string(),
                UserState::Active,
                password_hash,
                None,
                None
            );
            
            user_gateway.save_user(&user).await;
            
            role_gateway.link_role_to_user(
                &role.id,
                &user.id
            ).await;
            
            init_state_gateway.set_state(&Utc::now()).await;
            
            log::info!("Control account created!");
            log::info!("Login: control");
            log::info!("Password: {}", password);
        }
    }
}
