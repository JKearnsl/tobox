use strum_macros::{Display, EnumIter};
use crate::domain::models::r#box::BoxId;

#[derive(Display, EnumIter)]
pub enum Permission {
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
