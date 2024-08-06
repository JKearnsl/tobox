use core::option::Option;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::adapters::database::models::init_state::InitState;
use crate::adapters::database::pool::DbPool;
use crate::application::common::init_state_gateway::InitStateGateway as InitStateGatewayTrait;

pub struct InitStateGateway{
    pub db: DbPool,
}

impl InitStateGateway {
    pub fn new(db: DbPool) -> Self {
        InitStateGateway {
            db,
        }
    }
}

#[async_trait]
impl InitStateGatewayTrait for InitStateGateway {
    async fn get_state(&self) -> Option<DateTime<Utc>> {
        let row: Option<InitState> = sqlx::query_as("SELECT start_date FROM init_state")
            .fetch_optional(&self.db).await.unwrap();
        row.map(|r| r.start_date)
    }

    async fn set_state(&self, state: &DateTime<Utc>) {
        sqlx::query("
            INSERT INTO init_state (id, start_date) VALUES (1, $1) 
            ON CONFLICT (id) DO UPDATE SET start_date = $1
        ").bind(state).execute(&self.db).await.unwrap();
    }
}
