use uuid::Uuid;

use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::{NodePermission, PermissionTag, PermissionTextId};
use crate::domain::models::r#box::BoxId;
use crate::domain::models::session::SessionId;
use crate::domain::models::user::UserState;

pub struct AccessService {}

impl AccessService {
    
    pub fn ensure_can_create_box(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::CreateBox.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_box(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::GetBox.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_delete_box(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::DeleteSpecificBox(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::DeleteBox.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_update_box(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::UpdateSpecificBox(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::UpdateBox.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_create_object(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::CreateSpecificObject(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::CreateObject.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_object(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::GetSpecificObject(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::GetObject.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_delete_object(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::DeleteSpecificObject(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::DeleteObject.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    
    pub fn ensure_can_update_object(
        &self,
        is_auth: &bool,
        box_id: &BoxId,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&NodePermission::UpdateSpecificObject(*box_id).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&NodePermission::UpdateObject.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_user_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(&PermissionTag::GetUserSelf.to_string()) &&
            user_state.unwrap() != &UserState::Inactive
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_user(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_id: &Uuid,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if permissions.contains(&PermissionTag::GetUser.to_string()) {
            return Ok(())
        }
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(&PermissionTag::GetUserSelf.to_string()) &&
            user_id.unwrap() == get_user_id &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_users(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_ids: &Vec<Uuid>,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if permissions.contains(&PermissionTag::GetUser.to_string()) {
            return Ok(())
        }

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if
            permissions.contains(&PermissionTag::GetUserSelf.to_string()) && 
            get_user_ids.len() == 1 &&
            get_user_ids.contains(&user_id.unwrap()) &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_user_range(
        &self,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if permissions.contains(&PermissionTag::GetUser.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }


    pub fn ensure_can_update_user(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active && 
            permissions.contains(&PermissionTag::UpdateUser.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_update_user_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::UpdateUserSelf.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_reset_password(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if *is_auth || !permissions.contains(&PermissionTag::ResetUserPassword.to_string())
        {
            return Err(DomainError::AccessDenied)
        }

        Ok(())
    }


    pub fn ensure_can_confirm_user(
        &self,
        is_auth: &bool,
        permissions: &Vec<PermissionTextId>
    ) -> Result<(), DomainError> {
        if !permissions.contains(&PermissionTag::ConfirmUser.to_string()) || *is_auth {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_send_confirm_code(
        &self,
        is_auth: &bool,
        permissions: &Vec<PermissionTextId>
    ) -> Result<(), DomainError> {
        if !permissions.contains(&PermissionTag::SendConfirmCode.to_string()) || *is_auth {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_delete_session(
        &self,
        is_auth: &bool,
        user_session_id: Option<&SessionId>,
        del_session_id: &SessionId,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            permissions.contains(&PermissionTag::DeleteSession.to_string()) ||
            (
                permissions.contains(&PermissionTag::DeleteSessionSelf.to_string()) &&
                user_session_id.unwrap() == del_session_id &&
                user_state.unwrap() != &UserState::Inactive
            )
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_delete_session_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            user_state.unwrap() != &UserState::Inactive &&
            permissions.contains(&PermissionTag::DeleteSessionSelf.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_create_session(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth && permissions.contains(&PermissionTag::CreateSession.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_session(
        &self,
        is_auth: &bool,
        user_session_id: Option<&SessionId>,
        get_session_id: &SessionId,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            permissions.contains(&PermissionTag::GetSession.to_string()) ||
            (
                permissions.contains(&PermissionTag::GetSessionSelf.to_string()) &&
                user_session_id.unwrap() == get_session_id &&
                user_state.unwrap() != &UserState::Inactive
            )
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_sessions(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_id: &Uuid,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::GetSession.to_string()) {
            return Ok(())
        }
        
        if
            permissions.contains(&PermissionTag::GetSessionSelf.to_string()) &&
            get_user_id == user_id.unwrap() &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    

    pub fn ensure_can_get_session_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(&PermissionTag::GetSessionSelf.to_string()) &&
            user_state.unwrap() != &UserState::Inactive
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_access_log_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::GetAccessLogSelf.to_string()) &&
            user_state.unwrap() == &UserState::Active 
        {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_access_log(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::GetAccessLog.to_string()) &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_create_role(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::CreateRole.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_set_default_role(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if
        user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::SetDefaultRole.to_string())
        {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_delete_role(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::DeleteRole.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_role(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::GetRole.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_link_role_user(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::LinkUserRole.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_user_roles(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if
        user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::GetUserRole.to_string())
        {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_update_role(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(&PermissionTag::UpdateRole.to_string())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_permissions(
        &self,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if permissions.contains(&PermissionTag::GetPermission.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_update_permission(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::UpdatePermission.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_link_permission(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::LinkRolePermission.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
}
