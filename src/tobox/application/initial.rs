use chrono::Utc;
use strum::IntoEnumIterator;

use crate::application::common::hasher::Hasher;
use crate::application::common::init_state_gateway::InitStateGateway;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::config::CredentialsConfig;
use crate::domain::models::permission::{NodePermission, Permission, PermissionTextId};
use crate::domain::services::permission::PermissionService;
use crate::domain::services::role::RoleService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

pub async fn permissions(
    permission_gateway: &dyn PermissionGateway,
    permission_service: &PermissionService,
) {
    let permission_text_ids = [
        NodePermission::GetUserSelf,
        NodePermission::GetUser,
        NodePermission::CreateUser,
        NodePermission::UpdateUser,
        NodePermission::DeleteUser,

        NodePermission::GetRole,
        NodePermission::CreateRole,
        NodePermission::UpdateRole,
        NodePermission::DeleteRole,
        NodePermission::SetDefaultRole,
        NodePermission::LinkUserRole,
        NodePermission::GetUserRole,
        NodePermission::GetSelfRole,
        NodePermission::GetPermission,
        NodePermission::LinkRolePermission,

        NodePermission::GetBox,
        NodePermission::CreateBox,
        NodePermission::UpdateBox,
        NodePermission::DeleteBox,

        NodePermission::GetObject,
        NodePermission::CreateObject,
        NodePermission::UpdateObject,
        NodePermission::DeleteObject,
    ].map(|permission| {
        permission.to_string()
    }).collect::<Vec<String>>();

    let permission_text_ids_from_repo = permission_gateway.get_permissions().await.iter().map(|permission| {
        permission.text_id.clone()
    }).collect::<Vec<PermissionTextId>>();

    let permissions_to_add = permission_text_ids.iter().filter(
        |permission_text_id| {
            !permission_text_ids_from_repo.contains(permission_text_id)
        }
    ).map(|permission_text_id| {
        permission_service.create_permission(permission_text_id)
    }).collect::<Vec<Permission>>();

    permission_gateway.save_permissions(&permissions_to_add).await;
    log::info!("Permissions initialized!");
}

pub async fn control_account(
    role_gateway: &dyn RoleGateway,
    role_service: &RoleService,
    permission_gateway: &dyn PermissionGateway,
    user_gateway: &dyn UserGateway,
    user_service: &UserService,
    password_hasher: &dyn Hasher,
    validator: &ValidatorService,
    init_state_gateway: &dyn InitStateGateway,
    credentials: &Option<CredentialsConfig>,
) {
    if credentials.is_none() {
        log::debug!("Credentials not set");
        return;
    }
    
    let credentials = credentials.as_ref().unwrap();
    
    if let Err(error) = validator.validate_username(&credentials.username) {
        log::error!("Control account username validation error: {}", error);
        std::process::exit(1);
    }
    
    if credentials.password.is_some() && credentials.hashed_password.is_some() {
        log::error!(
            "Control account password and hashed_password are both set: \
            only one of them should be set"
        );
        std::process::exit(1);
    }
    
    if credentials.password.is_none() && credentials.hashed_password.is_none() {
        log::error!(
            "Control account password and hashed_password are both not set: \
            one of them should be set"
        );
        std::process::exit(1);
    }
    
    let password_hash = match &credentials.hashed_password {
        Some(hashed_password) => hashed_password.clone(),
        None => {
            let password = credentials.password.as_ref().unwrap();
            if let Err(error) = validator.validate_password(password) {
                log::error!("Control account password validation error: {}", error);
                std::process::exit(1);
            }
            password_hasher.hash(password).await
        }
    };
    
    // Create control user
    
    let user = match user_gateway.get_user_by_username_not_sensitive(&credentials.username).await {
        Some(user) => {
            if user.hashed_password != password_hash {
                let updated_user = user_service.update_user(
                    user.clone(),
                    user.username.clone(),
                    password_hash
                );
                user_gateway.save_user(&updated_user).await;
                log::info!("Control account updated!");
                user
            }
            else {
                user
            }
        },
        None => {
            let user = user_service.create_user(
                credentials.username.clone(),
                password_hash
            );
            user_gateway.save_user(&user).await;
            log::info!("Control account created!");
            user
        }
    };
    
    // Create control role
    
    match init_state_gateway.get_state().await {
        Some(_) => return,
        None => ()
    };
    
    let role = role_service.create_role(
        "Control".to_string(),
        Some("Control role".to_string())
    );

    let permission_text_ids = [
        NodePermission::GetUserSelf,
        NodePermission::GetUser,
        NodePermission::CreateUser,
        NodePermission::UpdateUser,
        NodePermission::DeleteUser,
        
        NodePermission::GetRole,
        NodePermission::CreateRole,
        NodePermission::UpdateRole,
        NodePermission::DeleteRole,
        NodePermission::SetDefaultRole,
        NodePermission::LinkUserRole,
        NodePermission::GetUserRole,
        NodePermission::GetSelfRole,
        NodePermission::GetPermission,
        NodePermission::LinkRolePermission,
        
        NodePermission::GetBox,
        NodePermission::CreateBox,
        NodePermission::UpdateBox,
        NodePermission::DeleteBox,
        
        NodePermission::GetObject,
        NodePermission::CreateObject,
        NodePermission::UpdateObject,
        NodePermission::DeleteObject,
        
    ].map(|permission| {
        permission.to_string()
    }).collect::<Vec<PermissionTextId>>();

    let permissions = match permission_gateway.get_permissions_by_text_ids(
        &permission_text_ids
    ).await {
        Some(permissions) => permissions,
        None => panic!("Permissions not found in database, was the permissions initialization?")
    };
    role_gateway.save_role(&role).await;

    permission_gateway.link_permissions_to_role(
        &role.id,
        &permissions.iter().map(
            |permission| permission.id.clone()
        ).collect::<Vec<_>>()
    ).await;

    role_gateway.link_role_to_user(
        &role.id,
        &user.id
    ).await;

    init_state_gateway.set_state(&Utc::now()).await;

    log::info!("Control role created!");
}
