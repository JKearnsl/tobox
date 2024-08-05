use strum_macros::Display;
use crate::domain::models::r#box::BoxId;

pub type PermissionId = String;

pub struct Permission {
    pub id: PermissionId,
    pub tag: PermissionTag,
}

#[derive(Display, Debug, Clone)]
pub enum PermissionTag {
    GetUser,
    CreateUser,
    DeleteUser,
    
    CreateSession,
    DeleteSession,
    
    GetRole,
    CreateRole,
    UpdateRole,
    DeleteRole,
    SetDefaultRole,
    LinkUserRole,
    GetUserRole,
    
    GetPermission,
    LinkRolePermission,
    
    GetBox,
    CreateBox,
    DeleteBox,
    
    #[strum(serialize = "DeleteSpecificBox({0})")]
    DeleteSpecificBox(BoxId),
    
    GetObject,
    CreateObject,
    UpdateObject,
    DeleteObject,

    #[strum(serialize = "GetSpecificObject({0})")]
    GetSpecificObject(BoxId),
    #[strum(serialize = "CreateSpecificObject({0})")]
    CreateSpecificObject(BoxId),
    #[strum(serialize = "UpdateSpecificObject({0})")]
    UpdateSpecificObject(BoxId),
    #[strum(serialize = "DeleteSpecificObject({0})")]
    DeleteSpecificObject(BoxId)
}


impl PermissionTag {
    pub fn static_tags() -> Vec<PermissionTag> {
        vec![
            PermissionTag::GetUser,
            PermissionTag::CreateUser,
            PermissionTag::DeleteUser,
            PermissionTag::CreateSession,
            PermissionTag::DeleteSession,
            PermissionTag::GetRole,
            PermissionTag::CreateRole,
            PermissionTag::UpdateRole,
            PermissionTag::DeleteRole,
            PermissionTag::SetDefaultRole,
            PermissionTag::LinkUserRole,
            PermissionTag::GetUserRole,
            PermissionTag::GetPermission,
            PermissionTag::LinkRolePermission,
            PermissionTag::GetBox,
            PermissionTag::CreateBox,
            PermissionTag::DeleteBox,
            PermissionTag::GetObject,
            PermissionTag::CreateObject,
            PermissionTag::UpdateObject,
            PermissionTag::DeleteObject
        ]
    }
    
    pub fn guest_tags() -> Vec<PermissionTag> {
        vec![
            PermissionTag::CreateSession,
        ]
    }
}
