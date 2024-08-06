use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::role::RoleId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: RoleId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl CreateIFNotExists for Role {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS roles (
                id CHAR(16) PRIMARY KEY, 
                title VARCHAR(64) NOT NULL, 
                description VARCHAR(256),  
                created_at TIMESTAMP WITH TIME ZONE NOT NULL, 
                updated_at TIMESTAMP WITH TIME ZONE
            );")
        .execute(&db_pool)
        .await?;
        Ok(())
    }
}