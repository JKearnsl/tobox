use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::permission::PermissionId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Permission {
    pub id: PermissionId,
    pub tag: String
}

impl CreateIFNotExists for Permission {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS permissions (
                id CHAR(16) PRIMARY KEY,
                tag VARCHAR(256) UNIQUE NOT NULL
            );")
            .execute(&db_pool)
            .await?;
        Ok(())
    }
}
