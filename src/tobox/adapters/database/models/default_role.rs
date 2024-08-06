use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::role::RoleId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct DefaultRole {
    pub id: RoleId,
}

impl CreateIFNotExists for DefaultRole {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS default_role (id VARCHAR(16) PRIMARY KEY);")
        .execute(&db_pool)
        .await?;
        Ok(())
    }
}