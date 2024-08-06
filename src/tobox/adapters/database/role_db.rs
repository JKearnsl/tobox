use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::models::roles::Role;
use crate::adapters::database::pool::DbPool;
use crate::application::common::role_gateway::{RoleGateway as RoleGatewayTrait, RoleLinker, RoleReader, RoleRemover, RoleWriter};
use crate::domain::models::role::{Role as RoleDomain, RoleId};
use crate::domain::models::user::UserId;

pub struct RoleGateway{
    db: DbPool,
}

impl RoleGateway {
    pub fn new(db: DbPool) -> Self {
        RoleGateway {
            db,
        }
    }
}

#[async_trait]
impl RoleReader for RoleGateway {
    async fn get_role(&self, role_id: &RoleId) -> Option<RoleDomain> {
        let row: Option<Role> = sqlx::query_as("SELECT * FROM roles WHERE id = $1")
            .bind(role_id)
            .fetch_optional(&self.db).await?;

        match row {
            None => None,
            Some(row) => Some(map_role_model_to_domain(row))
        }
    }

    async fn get_roles_range(&self, limit: &u64, offset: &u64) -> Vec<RoleDomain> {
        let rows: Vec<Role> = sqlx::query_as("SELECT * FROM roles LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db).await.unwrap();
        
        rows.into_iter().map(|row| map_role_model_to_domain(row)).collect()
    }

    async fn get_user_roles(&self, user_id: &UserId) -> Vec<RoleDomain> {
        let raw_sql = r#"
            SELECT
                role.*
            FROM
                role
            JOIN
                role_user ON role.id = role_user.role_id
            WHERE
                role_user.user_id = $1;
        "#;

        let rows: Vec<Role> = sqlx::query_as(raw_sql)
            .bind(user_id)
            .fetch_all(&self.db).await.unwrap();
        
        rows.into_iter().map(|row| map_role_model_to_domain(row)).collect()
    }
    
    async fn get_role_by_title_not_sensitive(&self, title: &String) -> Option<RoleDomain> {
        let row: Option<Role> = sqlx::query_as(
            "SELECT * FROM roles WHERE title = $1 COLLATE NOCASE"
        ).bind(title).fetch_optional(&self.db).await?;

        match row {
            None => None,
            Some(row) => Some(map_role_model_to_domain(row))
        }
    }

    async fn get_default_role(&self) -> Option<RoleDomain> {
        let row: Option<Role> = sqlx::query_as(
            "SELECT * FROM roles WHERE id = (SELECT id FROM default_role)"
        ).fetch_optional(&self.db).await?;

        match row {
            None => None,
            Some(row) => Some(map_role_model_to_domain(row))
        }
    }
}

#[async_trait]
impl RoleWriter for RoleGateway {
    async fn save_role(&self, role: &RoleDomain) {
        match sqlx::query_as("SELECT id FROM roles WHERE id = $1")
            .bind(&role.id)
            .fetch_optional(&self.db).await.unwrap() {
            Some(_) => {
                sqlx::query(
                    "UPDATE roles SET title = $1, description = $2 created_at = $3, updated_at = $4 \
                    WHERE id = $5"
                )
                    .bind(&role.title)
                    .bind(&role.description)
                    .bind(&role.created_at)
                    .bind(&role.updated_at)
                    .bind(&role.id)
                    .execute(&self.db)
                    .await
                    .unwrap();
            },
            None => {
                sqlx::query(
                    "INSERT INTO roles (id, title, description, created_at, updated_at) \
                    VALUES ($1, $2, $3, $4, $5)"
                )
                    .bind(&role.id)
                    .bind(&role.title)
                    .bind(&role.description)
                    .bind(&role.created_at)
                    .bind(&role.updated_at)
                    .execute(&self.db)
                    .await
                    .unwrap();
            }
        }
    }

    async fn set_default_role(&self, role_id: &RoleId) {
        sqlx::query(
            "UPDATE default_role SET id = $1"
        )
            .bind(role_id)
            .execute(&self.db)
            .await
            .unwrap();
    }
}

#[async_trait]
impl RoleLinker for RoleGateway {
    async fn link_role_to_user(&self, role_id: &RoleId, user_id: &UserId) {
        sqlx::query(
            "INSERT INTO role_user (role_id, user_id) VALUES ($1, $2)"
        )
            .bind(role_id)
            .bind(user_id)
            .execute(&self.db)
            .await
            .unwrap();
    }

    async fn unlink_role_from_user(&self, role_id: &RoleId, user_id: &UserId) {
        sqlx::query(
            "DELETE FROM role_user WHERE role_id = $1 AND user_id = $2"
        )
            .bind(role_id)
            .bind(user_id)
            .execute(&self.db)
            .await
            .unwrap();
    }

    async fn is_role_linked_to_user(&self, role_id: &RoleId, user_id: &UserId) -> bool {
        match sqlx::query_as(
            "SELECT role_id FROM role_user WHERE role_id = $1 AND user_id = $2"
        )
            .bind(role_id)
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .unwrap() {
            None => false,
            Some(_) => true
        }
    }
}

#[async_trait]
impl RoleRemover for RoleGateway {
    async fn remove_role(&self, role_id: &RoleId) {
        sqlx::query(
            "DELETE FROM roles WHERE id = $1"
        )
            .bind(role_id)
            .execute(&self.db)
            .await
            .unwrap();
    }
}

fn map_role_model_to_domain(role: Role) -> RoleDomain {
    RoleDomain {
        id: role.id,
        title: role.title,
        description: role.description,
        created_at: role.created_at,
        updated_at: role.updated_at
    }
}


impl RoleGatewayTrait for RoleGateway {}
