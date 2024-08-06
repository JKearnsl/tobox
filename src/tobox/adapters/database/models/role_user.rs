use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::role::RoleId;
use crate::domain::models::user::UserId;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct RoleUser {
    pub role_id: RoleId,
    pub user_id: UserId
}

impl CreateIFNotExists for RoleUser {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS role_user (
                role_id CHAR(16) NOT NULL REFERENCES roles(id), 
                user_id CHAR(16) NOT NULL REFERENCES users(id), 
                PRIMARY KEY (role_id, user_id)
            );")
            .execute(&db_pool)
            .await?;
        Ok(())
    }
}