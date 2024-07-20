use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait InitStateGateway {
    async fn get_state(&self) -> Option<DateTime<Utc>>;
    async fn set_state(&self, state: &DateTime<Utc>);
}
