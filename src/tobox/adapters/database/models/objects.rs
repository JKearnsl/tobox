use std::collections::HashMap;
use std::io::Bytes;
use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::object::ObjectId;
use crate::domain::models::r#box::BoxId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Object {
    pub id: ObjectId,
    pub name: Option<String>,
    pub hash: String,
    pub size: u64,
    pub content_type: String,
    pub metadata: Vec<u8>,
    pub box_id: BoxId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

impl CreateIFNotExists for ObjectId {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS boxes (
                id CHAR(25) PRIMARY KEY,
                name VARCHAR(256) NULL,
                hash VARCHAR(256) NOT NULL,
                size BIGINT NOT NULL,
                content_type VARCHAR(256) NOT NULL,
                metadata BLOB NOT NULL,
                box_id CHAR(16) NOT NULL REFERENCES boxes(id),
                created_at DATETIME NOT NULL,
                updated_at DATETIME ZONE NULL
            );")
        .execute(&db_pool)
        .await?;
        Ok(())
    }
}
