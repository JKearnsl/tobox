use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;

use crate::adapters::database::models::access_logs;
use crate::application::common::access_log_gateway::{
    AccessLogGateway as AccessLogGatewayTrait,
    AccessLogReader,
    AccessLogWriter
};
use crate::domain::models::access_log::AccessLog as AccessLogDomain;
use crate::domain::models::user::UserId;

pub struct AccessLogGateway{
    pub db: Box<DbConn>,
}

impl AccessLogGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        AccessLogGateway {
            db,
        }
    }
}

#[async_trait]
impl AccessLogReader for AccessLogGateway {

    async fn get_user_records(&self, user_id: &UserId, limit: &u64, offset: &u64) -> Vec<AccessLogDomain> {
        let records = access_logs::Entity::find()
            .filter(Expr::col(access_logs::Column::UserId).eq(user_id.clone()))
            .limit(*limit)
            .offset(*offset)
            .all(self.db.as_ref())
            .await
            .unwrap();

        records.iter().map(|record| map_rec_model_to_domain(record.clone())).collect()
    }
}

#[async_trait]
impl AccessLogWriter for AccessLogGateway {
    async fn save_rec(&self, data: &AccessLogDomain) {
        let model = map_rec_domain_to_model(data.clone());
        
        match access_logs::Entity::find_by_id(data.id).one(self.db.as_ref()).await.unwrap() {
            Some(_) => {
                access_logs::Entity::update(model).exec(self.db.as_ref()).await.unwrap();
            }
            None => {
                access_logs::Entity::insert(model).exec(self.db.as_ref()).await.unwrap();
            }
        }
    }
}

fn map_rec_model_to_domain(access_rec: access_logs::Model) -> AccessLogDomain {
    AccessLogDomain {
        id: access_rec.id,
        user_id: access_rec.user_id,
        is_success: access_rec.is_success,
        ip: access_rec.ip,
        client: access_rec.client,
        os: access_rec.os,
        device: access_rec.device,
        created_at: access_rec.created_at,
    }
}

fn map_rec_domain_to_model(access_rec: AccessLogDomain) -> access_logs::ActiveModel {
    access_logs::ActiveModel {
        id: Set(access_rec.id),
        user_id: Set(access_rec.user_id),
        is_success: Set(access_rec.is_success),
        ip: Set(access_rec.ip),
        client: Set(access_rec.client),
        os: Set(access_rec.os),
        device: Set(access_rec.device),
        created_at: Set(access_rec.created_at),
    }
}


impl AccessLogGatewayTrait for AccessLogGateway {}
