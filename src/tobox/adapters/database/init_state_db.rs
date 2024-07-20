use core::option::Option;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{DbConn, EntityTrait};
use sea_orm::ActiveValue::Set;

use crate::adapters::database::models::init_state;
use crate::application::common::init_state_gateway::InitStateGateway as InitStateGatewayTrait;

pub struct InitStateGateway{
    pub db: Box<DbConn>,
}

impl InitStateGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        InitStateGateway {
            db,
        }
    }
}

#[async_trait]
impl InitStateGatewayTrait for InitStateGateway {
    async fn get_state(&self) -> Option<DateTime<Utc>> {
        init_state::Entity::find().one(&*self.db).await.unwrap().map(|state| state.start_date)
    }

    async fn set_state(&self, state: &DateTime<Utc>) {
        let state = init_state::ActiveModel {
            start_date: Set(state.clone()),
        };
        init_state::Entity::delete_many().exec(&*self.db).await.unwrap();
        init_state::Entity::insert(state).exec(&*self.db).await.unwrap();
    }
}