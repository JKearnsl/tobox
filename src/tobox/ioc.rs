use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::database::permission_db::PermissionGateway;
use crate::adapters::database::role_db::RoleGateway;
use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
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
use crate::application::role::get_by_ids::GetRolesByIds;
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
use crate::domain::services::access::AccessService;
use crate::domain::services::object::ObjectService;
use crate::domain::services::permission::PermissionService;
use crate::domain::services::r#box::BoxService;
use crate::domain::services::role::RoleService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::node::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    user_service: UserService,

    session_gateway: SessionGateway,
    session_service: SessionService,

    role_gateway: RoleGateway,
    role_service: RoleService,

    permission_gateway: PermissionGateway,
    permission_service: PermissionService,
    
    box_gateway: BoxGateway,
    box_service: BoxService,
    
    object_gateway: ObjectGateway,
    object_service: ObjectService,

    file_storage_gateway: FileStorageGateway,

    password_hasher: Argon2PasswordHasher,

    validator: ValidatorService,
    access_service: AccessService,
}

impl IoC {
    pub fn new(
        session_ttl: u32,
    ) -> Self {
        Self {
            user_gateway: UserGateway::new(),
            user_service: UserService{},

            session_gateway: SessionGateway::new(),
            session_service: SessionService::new(session_ttl),

            role_gateway: RoleGateway::new(),
            role_service: RoleService{},

            permission_gateway: PermissionGateway::new(),
            permission_service: PermissionService{},
            
            box_gateway: BoxGateway::new(),
            box_service: BoxService::new(),
            
            object_gateway: ObjectGateway::new(),
            object_service: ObjectService::new(),
            
            file_storage_gateway: FileStorageGateway::new(),

            password_hasher: Argon2PasswordHasher::new(),

            validator: ValidatorService::new(),
            access_service: AccessService{},
        }
    }
}

impl InteractorFactory for IoC {
    fn get_user_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetUserById {
        GetUserById {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_user_range(&self, id_provider: Box<dyn IdProvider>) -> GetUserRange {
        GetUserRange {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf {
        GetUserSelf {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser {
        CreateUser {
            user_gateway: &self.user_gateway,
            role_gateway: &self.role_gateway,
            user_service: &self.user_service,
            id_provider,
            password_hasher: &self.password_hasher,
            validator: &self.validator,
            access_service: &self.access_service,
        }
    }

    fn delete_user(&self, id_provider: Box<dyn IdProvider>) -> DeleteUser {
        DeleteUser {
            user_gateway: &self.user_gateway,
            id_provider,
            validator: &self.validator,
            access_service: &self.access_service,
        }
    }

    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession {
        CreateSession {
            session_writer: &self.session_gateway,
            session_service: &self.session_service,
            user_reader: &self.user_gateway,
            id_provider,
            password_hasher: &self.password_hasher,
            validator: &self.validator,
            access_service: &self.access_service,
            permission_reader: &self.permission_gateway,
        }
    }

    fn create_role(&self, id_provider: Box<dyn IdProvider>) -> CreateRole {
        CreateRole {
            role_gateway: &self.role_gateway,
            permission_gateway: &self.permission_gateway,
            role_service: &self.role_service,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn get_role_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetRoleById {
        GetRoleById {
            role_reader: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_roles_by_ids(&self, id_provider: Box<dyn IdProvider>) -> GetRolesByIds {
        GetRolesByIds {
            role_gateway: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_role_by_user(&self, id_provider: Box<dyn IdProvider>) -> GetUserRoles {
        GetUserRoles {
            role_reader: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
            user_reader: &self.user_gateway,
        }
    }

    fn get_role_range(&self, id_provider: Box<dyn IdProvider>) -> GetRoleRange {
        GetRoleRange {
            role_reader: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn set_default_role(&self, id_provider: Box<dyn IdProvider>) -> SetDefaultRole {
        SetDefaultRole {
            role_gateway: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_default_role(&self, id_provider: Box<dyn IdProvider>) -> GetDefaultRole {
        GetDefaultRole {
            role_reader: &self.role_gateway,
            permission_reader: &self.permission_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn link_role_user(&self, id_provider: Box<dyn IdProvider>) -> LinkRoleUser {
        LinkRoleUser {
            role_gateway: &self.role_gateway,
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn unlink_role_user(&self, id_provider: Box<dyn IdProvider>) -> UnlinkRoleUser {
        UnlinkRoleUser {
            role_gateway: &self.role_gateway,
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn update_role(&self, id_provider: Box<dyn IdProvider>) -> UpdateRole {
        UpdateRole {
            role_gateway: &self.role_gateway,
            role_service: &self.role_service,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn delete_role(&self, id_provider: Box<dyn IdProvider>) -> DeleteRole {
        DeleteRole {
            role_gateway: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_permission_range(&self, id_provider: Box<dyn IdProvider>) -> GetPermissionRange {
        GetPermissionRange {
            permission_reader: &self.permission_gateway,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn get_role_permissions(&self, id_provider: Box<dyn IdProvider>) -> GetRolePermissions {
        GetRolePermissions {
            permission_reader: &self.permission_gateway,
            id_provider,
            access_service: &self.access_service,
            role_reader: &self.role_gateway,
        }
    }

    fn get_user_permissions(&self, id_provider: Box<dyn IdProvider>) -> GetUserPermissions {
        GetUserPermissions {
            permission_reader: &self.permission_gateway,
            id_provider,
            access_service: &self.access_service,
            user_reader: &self.user_gateway,
        }
    }

    fn link_role_permission(&self, id_provider: Box<dyn IdProvider>) -> LinkRolePermission {
        LinkRolePermission {
            permission_gateway: &self.permission_gateway,
            role_reader: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn unlink_role_permission(&self, id_provider: Box<dyn IdProvider>) -> UnlinkRolePermission {
        UnlinkRolePermission {
            permission_gateway: &self.permission_gateway,
            role_reader: &self.role_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn create_box(&self, id_provider: Box<dyn IdProvider>) -> CreateBox {
        CreateBox {
            box_gateway: &self.box_gateway,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
            box_service: &self.box_service,
        }
    }

    fn delete_box(&self, id_provider: Box<dyn IdProvider>) -> DeleteBox {
        DeleteBox {
            box_gateway: &self.box_gateway,
            box_service: &self.box_service,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn get_box_range(&self, id_provider: Box<dyn IdProvider>) -> GetBoxRange {
        GetBoxRange {
            box_reader: &self.box_gateway,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn create_object(&self, id_provider: Box<dyn IdProvider>) -> CreateObject {
        CreateObject {
            box_reader: &self.box_gateway,
            file_storage_writer: &self.file_storage_gateway,
            object_gateway: &self.object_gateway,
            object_service: &self.object_service,
            id_provider,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn get_object(&self, id_provider: Box<dyn IdProvider>) -> GetObject {
        GetObject {
            file_storage_reader: &self.file_storage_gateway,
            object_reader: &self.object_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_object_info(&self, id_provider: Box<dyn IdProvider>) -> GetObjectInfo {
        GetObjectInfo {
            object_reader: &self.object_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }
}
