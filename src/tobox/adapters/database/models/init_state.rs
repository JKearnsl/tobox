use chrono::{DateTime, Utc};

use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct InitState {
    pub id: u32,
    pub start_date: DateTime<Utc>
}

impl CreateIFNotExists for InitState {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS init_state (
                id INTEGER PRIMARY KEY,
                start_date TIMESTAMP NOT NULL
            );")
            .execute(&db_pool)
            .await?;
        Ok(())
    }
}
