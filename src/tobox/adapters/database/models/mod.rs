use crate::adapters::database::pool::DbPool;

pub mod permissions;
pub mod role_permissions;
pub mod role_users;
pub mod roles;
pub mod users;
pub mod default_role;
pub mod init_state;

pub trait CreateIFNotExists {
    async fn create_if_not_exists(&self, db_pool: DbPool) -> Result<(), sqlx::Error>;
}
