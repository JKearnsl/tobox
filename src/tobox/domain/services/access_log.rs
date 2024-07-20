use crate::domain::models::access_log::{AccessLog, AccessLogId};
use crate::domain::models::user::UserId;

pub struct AccessLogService {}

impl AccessLogService {
    pub fn create_log(
        &self,
        user_id: UserId,
        is_success: bool,
        ip: String,
        client: String,
        os: String,
        device: String,
    ) -> AccessLog {
        AccessLog {
            id: AccessLogId::new_v4(),
            user_id,
            is_success,
            ip,
            client,
            os,
            device,
            created_at: chrono::Utc::now(),
        }
    }
}
