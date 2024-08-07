use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::r#box::BoxId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Box {
    pub id: BoxId,
    pub created_at: DateTime<Utc>,
}

impl CreateIFNotExists for Box {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS boxes (
                id CHAR(16) PRIMARY KEY,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL
            );")
        .execute(&db_pool)
        .await?;
        Ok(())
    }
}