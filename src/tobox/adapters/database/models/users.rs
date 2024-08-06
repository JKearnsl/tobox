use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
}

impl CreateIFNotExists for User {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id CHAR(16) PRIMARY KEY, 
                username VARCHAR(32) UNIQUE NOT NULL, 
                hashed_password VARCHAR(256) NOT NULL, 
                created_at TIMESTAMP WITH TIME ZONE NOT NULL
            );")
        .execute(&db_pool)
        .await?;
        Ok(())
    }
}