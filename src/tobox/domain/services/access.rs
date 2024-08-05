
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::PermissionTag;
use crate::domain::models::r#box::BoxId;

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
        
        if permissions.contains(&PermissionTag::CreateBox.to_string()) {
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
        
        if permissions.contains(&PermissionTag::GetBox.to_string()) {
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
        
        if permissions.contains(&PermissionTag::DeleteSpecificBox(box_id.clone()).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&PermissionTag::DeleteBox.to_string()) {
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
        
        if permissions.contains(&PermissionTag::CreateSpecificObject(box_id.clone()).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&PermissionTag::CreateObject.to_string()) {
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
        
        if permissions.contains(&PermissionTag::GetSpecificObject(box_id.clone()).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&PermissionTag::GetObject.to_string()) {
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
        
        if permissions.contains(&PermissionTag::DeleteSpecificObject(box_id.clone()).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&PermissionTag::DeleteObject.to_string()) {
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
        
        if permissions.contains(&PermissionTag::UpdateSpecificObject(box_id.clone()).to_string()) {
            return Ok(())
        }
        
        if permissions.contains(&PermissionTag::UpdateObject.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_create_user(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::CreateUser.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    
    pub fn ensure_can_get_user(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        

        if permissions.contains(&PermissionTag::GetUser.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_delete_user(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }


        if permissions.contains(&PermissionTag::DeleteUser.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_delete_session(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::DeleteSession.to_string()) {
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
    pub fn ensure_can_create_role(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::CreateRole.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_set_default_role(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::SetDefaultRole.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_delete_role(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::DeleteRole.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_role(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::GetRole.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_link_role_user(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::LinkUserRole.to_string()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_user_roles(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(&PermissionTag::GetUserRole.to_string()) {
            return Ok(())
        }

        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_update_role(
        &self,
        is_auth: &bool,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if permissions.contains(&PermissionTag::UpdateRole.to_string()) {
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
