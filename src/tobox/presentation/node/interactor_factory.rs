use crate::application::common::id_provider::IdProvider;
use crate::application::object::create::CreateObject;
use crate::application::object::get::GetObject;
use crate::application::object::get_info::GetObjectInfo;
use crate::application::permission::get_by_role::GetRolePermissions;
use crate::application::permission::get_by_user::GetUserPermissions;
use crate::application::permission::get_range::GetPermissionRange;
use crate::application::permission::link::LinkRolePermission;
use crate::application::permission::unlink::UnlinkRolePermission;
use crate::application::r#box::create::CreateBox;
use crate::application::r#box::delete::DeleteBox;
use crate::application::r#box::get_range::GetBoxRange;
use crate::application::role::create::CreateRole;
use crate::application::role::delete::DeleteRole;
use crate::application::role::get_by_id::GetRoleById;
use crate::application::role::get_by_user::GetUserRoles;
use crate::application::role::get_default::GetDefaultRole;
use crate::application::role::get_range::GetRoleRange;
use crate::application::role::link::LinkRoleUser;
use crate::application::role::set_default::SetDefaultRole;
use crate::application::role::unlink::UnlinkRoleUser;
use crate::application::role::update::UpdateRole;
use crate::application::session::create::CreateSession;
use crate::application::user::create::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_range::GetUserRange;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::delete::DeleteUser;


pub trait InteractorFactory {
    fn get_user_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetUserById;
    fn get_user_range(&self, id_provider: Box<dyn IdProvider>) -> GetUserRange;
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf;
    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser;
    fn delete_user(&self, id_provider: Box<dyn IdProvider>) -> DeleteUser;
    
    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession;
    
    fn create_role(&self, id_provider: Box<dyn IdProvider>) -> CreateRole;
    fn get_role_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetRoleById;
    fn get_role_by_user(&self, id_provider: Box<dyn IdProvider>) -> GetUserRoles;
    fn get_role_range(&self, id_provider: Box<dyn IdProvider>) -> GetRoleRange;
    fn set_default_role(&self, id_provider: Box<dyn IdProvider>) -> SetDefaultRole;
    fn get_default_role(&self, id_provider: Box<dyn IdProvider>) -> GetDefaultRole;
    fn link_role_user(&self, id_provider: Box<dyn IdProvider>) -> LinkRoleUser;
    fn unlink_role_user(&self, id_provider: Box<dyn IdProvider>) -> UnlinkRoleUser;
    fn update_role(&self, id_provider: Box<dyn IdProvider>) -> UpdateRole;
    fn delete_role(&self, id_provider: Box<dyn IdProvider>) -> DeleteRole;
    
    fn get_permission_range(&self, id_provider: Box<dyn IdProvider>) -> GetPermissionRange;
    fn get_role_permissions(&self, id_provider: Box<dyn IdProvider>) -> GetRolePermissions;
    fn get_user_permissions(&self, id_provider: Box<dyn IdProvider>) -> GetUserPermissions;
    fn link_role_permission(&self, id_provider: Box<dyn IdProvider>) -> LinkRolePermission;
    fn unlink_role_permission(&self, id_provider: Box<dyn IdProvider>) -> UnlinkRolePermission;
    
    fn create_box(&self, id_provider: Box<dyn IdProvider>) -> CreateBox;
    fn delete_box(&self, id_provider: Box<dyn IdProvider>) -> DeleteBox;
    fn get_box_range(&self, id_provider: Box<dyn IdProvider>) -> GetBoxRange;
    
    fn create_object(&self, id_provider: Box<dyn IdProvider>) -> CreateObject;
    fn get_object(&self, id_provider: Box<dyn IdProvider>) -> GetObject;
    fn get_object_info(&self, id_provider: Box<dyn IdProvider>) -> GetObjectInfo;
    
}
