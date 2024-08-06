use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::models::permissions::Permission;
use crate::adapters::database::models::role_permissions;
use crate::adapters::database::pool::DbPool;
use crate::application::common::permission_gateway::{
    PermissionGateway as PermissionGatewayTrait,
    PermissionLinker,
    PermissionReader,
    PermissionRemover,
    PermissionWriter
};
use crate::domain::models::permission::{
    Permission as PermissionDomain,
    PermissionId,
    PermissionTag
};
use crate::domain::models::role::RoleId;
use crate::domain::models::user::UserId;

pub struct PermissionGateway{
    db: DbPool
}

impl PermissionGateway {
    pub fn new(db: DbPool) -> Self {
        PermissionGateway {
            db,
        }
    }
}

#[async_trait]
impl PermissionReader for PermissionGateway {
    async fn get_permission(&self, permission_id: &PermissionId) -> Option<PermissionDomain> {
        match sqlx::query_as::<_, Permission>(
            "SELECT permissions.* FROM permissions WHERE id = $1"
        ).bind(permission_id).fetch_optional(&*self.db).await.unwrap() {
            Some(permission) => Some(PermissionDomain {
                id: permission.id,
                tag: PermissionTag::from(permission.tag)
            }),
            None => None
        }
    }
    
    async fn get_permissions(&self, permission_ids: &Vec<PermissionId>) -> Option<Vec<PermissionDomain>> {
        let permissions: Vec<Permission> = sqlx::query_as(
            "SELECT * FROM permissions WHERE id = ANY($1)"
        ).bind(permission_ids.iter().map(
            |el| el.clone() as i64)
            .collect()
        ).fetch_all(&self.db).await?;
        
        if permissions.len() != permission_ids.len() {
            return None
        }
        
        Some(permissions.into_iter().map(|permission| PermissionDomain {
            id: permission.id,
            tag: PermissionTag::from(permission.tag)
        }).collect())
    }

    async fn get_permissions_by_tags(&self, permission_text_ids: &Vec<PermissionTag>) -> Option<Vec<PermissionDomain>> {
        let permissions: Vec<Permission> = sqlx::query_as(
            "SELECT * FROM permissions WHERE tag = ANY($1)"
        ).bind(permission_text_ids.iter().map(
            |el| el.clone().to_string())
            .collect()
        ).fetch_all(&self.db).await?;
        
        if permissions.len() != permission_text_ids.len() {
            return None
        }
        
        Some(permissions.into_iter().map(
            |permission| PermissionDomain {
                id: permission.id,
                tag: PermissionTag::from(permission.tag)
            }
        ).collect())
    }

    async fn get_permissions_range(&self, limit: &u64, offset: &u64) -> Vec<PermissionDomain> {
        let permissions: Vec<Permission> = sqlx::query_as(
            "SELECT * FROM permissions LIMIT $1 OFFSET $2"
        ).bind(limit).bind(offset).fetch_all(&self.db).await.unwrap();
        permissions.into_iter().map(|permission| PermissionDomain {
            id: permission.id,
            tag: PermissionTag::from(permission.tag)
        }).collect()
    }

    async fn get_role_permissions(&self, role_id: &RoleId) -> Vec<PermissionDomain> {
        let raw_sql = r#"
            SELECT permission.* FROM permissions
            JOIN role_permissions ON permission.id = role_permissions.permission_id
            WHERE role_permissions.role_id = $1;
        "#;

        let rows = sqlx::query_as::<_, Permission>(raw_sql)
            .bind(role_id)
            .fetch_all(&*self.db)
            .await.unwrap();
        
        rows.into_iter().map(|permission| PermissionDomain {
            id: permission.id,
            tag: PermissionTag::from(permission.tag)
        }).collect()
    }

    async fn get_user_permissions(&self, user_id: &UserId) -> Vec<PermissionDomain> {
        let raw_sql = r#"
            SELECT permission.* FROM permissions
            JOIN role_permissions ON permission.id = role_permissions.permission_id
            JOIN role_user ON role_permissions.role_id = role_user.role_id
            WHERE role_user.user_id = $1;
        "#;

        let rows = sqlx::query_as::<_, Permission>(raw_sql)
            .bind(user_id)
            .fetch_all(&*self.db)
            .await.unwrap();
        
        rows.into_iter().map(|permission| PermissionDomain {
            id: permission.id,
            tag: PermissionTag::from(permission.tag)
        }).collect()
    }
}

#[async_trait]
impl PermissionWriter for PermissionGateway {
    async fn save_permission(&self, data: &PermissionDomain) {
        sqlx::query(
            "INSERT INTO permissions (id, tag) VALUES ($1, $2) 
            ON CONFLICT (id) DO UPDATE SET tag = $2"
        ).bind(data.id).bind(data.tag.to_string()).execute(&*self.db).await.unwrap();
    }

    async fn save_permissions(&self, data: &Vec<PermissionDomain>) {
        let mut query = sqlx::query(
            "INSERT INTO permissions (id, tag) VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET tag = EXCLUDED.tag"
        );
        for permission in data.iter() {
            query = query.bind(permission.id).bind(permission.tag.to_string());
        }
        query.execute(&*self.db).await.unwrap();
    }
}

#[async_trait]
impl PermissionRemover for PermissionGateway {
    async fn remove_permission(&self, permission_id: PermissionId) {
        sqlx::query("DELETE FROM permissions WHERE id = $1")
            .bind(permission_id)
            .execute(&*self.db)
            .await.unwrap();
    }
}

#[async_trait]
impl PermissionLinker for PermissionGateway {
    async fn is_permission_linked_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> bool {
        let row: Option<role_permissions::RolePermission> = sqlx::query_as(
            "SELECT * FROM role_permissions WHERE role_id = $1 AND permission_id = $2"
        ).bind(role_id).bind(permission_id).fetch_optional(&*self.db).await.unwrap();
        
        row.is_some()
    }

    async fn link_permission_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        sqlx::query(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)"
        ).bind(role_id).bind(permission_id).execute(&*self.db).await.unwrap();
    }

    async fn link_permissions_to_role(&self, role_id: &RoleId, permission_ids: &Vec<PermissionId>) {
        let mut query = sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES ");
        for permission_id in permission_ids.iter() {
            query = query.bind(role_id).bind(permission_id);
        }
        query.execute(&*self.db).await.unwrap();
    }

    async fn unlink_permission_from_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        sqlx::query(
            "DELETE FROM role_permissions WHERE role_id = $1 AND permission_id = $2"
        ).bind(role_id).bind(permission_id).execute(&*self.db).await.unwrap();
    }
}
impl PermissionGatewayTrait for PermissionGateway {}
