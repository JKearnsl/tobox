use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::permission::PermissionId;
use crate::domain::models::role::RoleId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct RolePermission {
    pub role_id: RoleId,
    pub permission_id: PermissionId
}

impl CreateIFNotExists for RolePermission {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS role_permissions (
                role_id CHAR(16) NOT NULL REFERENCES roles(id), 
                permission_id INTEGER NOT NULL REFERENCES permissions(id), 
                PRIMARY KEY (role_id, permission_id)
            );")
            .execute(&db_pool)
            .await?;
        Ok(())
    }
}