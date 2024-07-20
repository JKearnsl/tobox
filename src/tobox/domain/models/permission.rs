use strum_macros::{Display, EnumIter};

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
}
