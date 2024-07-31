use strum_macros::Display;
use uuid::Uuid;
use crate::domain::models::r#box::BoxId;

pub type PermissionId = Uuid;
pub type PermissionTextId = String;

pub struct Permission {
    pub id: PermissionId,
    pub text_id: PermissionTextId,
}

#[derive(Display)]
pub enum NodePermission {
    GetUserSelf,
    GetUser,
    CreateUser,
    UpdateUser,
    DeleteUser,
    
    GetRole,
    CreateRole,
    UpdateRole,
    DeleteRole,
    SetDefaultRole,
    LinkUserRole,
    GetUserRole,
    GetSelfRole,
    
    GetPermission,
    LinkRolePermission,
    
    GetBox,
    CreateBox,
    UpdateBox,
    DeleteBox,
    
    #[strum(serialize = "UpdateSpecificBox({0})")]
    UpdateSpecificBox(BoxId),
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
