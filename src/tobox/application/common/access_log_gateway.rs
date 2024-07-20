use async_trait::async_trait;

use crate::domain::models::access_log::AccessLog;
use crate::domain::models::user::UserId;

#[async_trait]
pub trait AccessLogReader {
    async fn get_user_records(&self, user_id: &UserId, limit: &u64, offset: &u64) -> Vec<AccessLog>;

}

#[async_trait]
pub trait AccessLogWriter {
    async fn save_rec(&self, data: &AccessLog);
}

pub trait AccessLogGateway: AccessLogReader + AccessLogWriter { }
