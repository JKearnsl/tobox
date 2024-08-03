use async_trait::async_trait;

use crate::domain::models::permission::{Permission as PermissionDomain, PermissionId, PermissionTextId};
use crate::domain::models::role::RoleId;
use crate::domain::models::service::ServiceId;
use crate::domain::models::user::UserId;

#[async_trait]
pub trait PermissionReader {
    async fn get_permission_by_id(&self, permission_id: &PermissionId) -> Option<PermissionDomain>;
    async fn get_permissions_by_service_id(&self, service_id: &ServiceId) -> Vec<PermissionDomain>;
    async fn get_permissions_by_ids(&self, permission_ids: &Vec<PermissionId>) -> Option<Vec<PermissionDomain>>;
    async fn get_permissions_by_text_ids(&self, permission_text_ids: &Vec<PermissionTextId>) -> Option<Vec<PermissionDomain>>;
    async fn get_permissions_range(&self, limit: &u64, offset: &u64) -> Vec<PermissionDomain>;
    async fn get_role_permissions(&self, role_id: &RoleId) -> Vec<PermissionDomain>;
    async fn get_user_permissions(&self, user_id: &UserId) -> Vec<PermissionDomain>;
}

#[async_trait]
pub trait PermissionWriter {
    async fn save_permission(&self, data: &PermissionDomain);
    async fn save_permissions(&self, data: &Vec<PermissionDomain>);
}

#[async_trait]
pub trait PermissionRemover {
    async fn remove_permission(&self, permission_id: PermissionId );
}

#[async_trait]
pub trait PermissionLinker {
    async fn is_permission_linked_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> bool;
    async fn link_permission_to_role(&self, role_id: &RoleId, permission_id: &PermissionId);
    async fn link_permissions_to_role(&self, role_id: &RoleId, permission_ids: &Vec<PermissionId>);
    async fn unlink_permission_from_role(&self, role_id: &RoleId, permission_id: &PermissionId);
}


pub trait PermissionGateway: PermissionReader + PermissionWriter + PermissionLinker + Send + Sync {}