use std::collections::HashMap;
use std::str::FromStr;

use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::cmd;
use sea_orm::{DbBackend, DbConn, EntityTrait, FromQueryResult, JsonValue, QueryFilter, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use crate::adapters::database::models::sessions;
use crate::application::common::session_gateway::{
    SessionGateway as SessionGatewayTrait,
    SessionReader,
    SessionRemover,
    SessionWriter
};
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::service::ServiceTextId;
use crate::domain::models::session::{
    Session,
    SessionId,
    SessionTokenHash
};
use crate::domain::models::user::{UserId, UserState};

pub struct SessionGateway {
    cache_redis_pool: Box<Pool>,
    cache_exp: u32,
    db: Box<DbConn>,
}

impl SessionGateway {
    pub fn new(
        redis_pool: Box<Pool>,
        cache_exp: u32,
        db: Box<DbConn>,
    ) -> Self {
        SessionGateway {
            cache_redis_pool: redis_pool,
            cache_exp,
            db,
        }
    }
}


#[async_trait]
impl SessionReader for SessionGateway {
    async fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        match sessions::Entity::find_by_id(session_id.clone())
            .one(&*self.db)
            .await.unwrap() {
            Some(model) => Some(map_session_model_to_domain(model)),
            None => None
        }
    }

    async fn get_session_by_token_hash(
        &self,
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, HashMap<ServiceTextId, Vec<PermissionTextId>>)> {
        let raw_sql = r#"
            SELECT
                sessions.*,
                user.state::text AS user_state,
                services.text_id AS service_text_id,
                permission.text_id AS permission_text_id
            FROM
                sessions
            JOIN
                user ON sessions.user_id = user.id
            JOIN
                role_user ON user.id = role_user.user_id
            JOIN
                role_permissions ON role_user.role_id = role_permissions.role_id
            JOIN
                permission ON role_permissions.permission_id = permission.id
            JOIN
                services ON permission.service_id = services.id
            WHERE
                sessions.token_hash = $1;
        "#;
        
        let raw_values: Vec<JsonValue> = JsonValue::find_by_statement(
            Statement::from_sql_and_values(
                DbBackend::Postgres,
                raw_sql,
                vec![token_hash.as_str().into()],
            )
        )
            .all(&*self.db)
            .await.unwrap();

        if raw_values.is_empty() {
            return None;
        }

        let session = serde_json::from_value::<Session>(raw_values[0].clone()).unwrap();
        let user_state: UserState = UserState::from_str(
            raw_values[0].get("user_state").unwrap().as_str().unwrap()
        ).unwrap();

        let mut data: HashMap<ServiceTextId, Vec<PermissionTextId>> = HashMap::new();
        raw_values.iter().for_each(
            |value| {
                let service_text_id: ServiceTextId = ServiceTextId::from_str(
                    value.get("service_text_id").unwrap().as_str().unwrap()
                ).unwrap();
                let permission_text_id: PermissionTextId = PermissionTextId::from_str(
                    value.get("permission_text_id").unwrap().as_str().unwrap()
                ).unwrap();
                
                data.entry(service_text_id).or_insert(Vec::new()).push(permission_text_id);
            }
        );
        
        Some((session, user_state, data))
    }

    async fn get_session_by_token_hash_from_cache(
        &self,
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, HashMap<ServiceTextId, Vec<PermissionTextId>>)> {
        let mut conn = self.cache_redis_pool.get().await.unwrap();
        match cmd("GET")
            .arg(token_hash.as_str())
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                Some(
                    serde_json::from_str::<(
                        Session,
                        UserState,
                        HashMap<ServiceTextId, Vec<PermissionTextId>>
                    )>(value.as_str()).unwrap()
                )
            },
            Err(_) => {
                None
            }
        }
    }

    async fn get_user_sessions(&self, user_id: &UserId) -> Vec<Session> {
        let sessions: Vec<sessions::Model> = sessions::Entity::find().filter(
            Expr::col(sessions::Column::UserId).eq(user_id.to_string())
        )
            .all(&*self.db)
            .await
            .unwrap();

        sessions.iter().map(
            |model| map_session_model_to_domain(model.clone())
        ).collect()
    }
}

#[async_trait]
impl SessionWriter for SessionGateway {
    async fn save_session(&self, data: &Session) {
        let session_model = sessions::ActiveModel {
            id: Set(data.id),
            token_hash: Set(data.token_hash.clone()),
            user_id: Set(data.user_id),
            ip: Set(data.ip.clone()),
            client: Set(data.client.clone()),
            os: Set(data.os.clone()),
            device: Set(data.device.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at.clone())
        };

        match sessions::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                sessions::Entity::update(session_model).exec(&*self.db).await.unwrap();
            },
            None => {
                sessions::Entity::insert(session_model).exec(&*self.db).await.unwrap();
            }
        }
    }

    async fn save_session_to_cache(
        &self,
        data: &Session,
        user_state: &UserState,
        permissions: &HashMap<ServiceTextId, Vec<PermissionTextId>>
    ) {
        let mut conn = self.cache_redis_pool.get().await.unwrap();
        
        let serde_json = serde_json::to_string(&(
            data.clone(),
            user_state.clone(),
            permissions.clone()
        )).unwrap();
        
        cmd("SET")
            .arg(data.token_hash.as_str())
            .arg(serde_json.as_str())
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
        cmd("EXPIRE")
            .arg(data.token_hash.as_str())
            .arg(self.cache_exp)
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
    }
}

#[async_trait]
impl SessionRemover for SessionGateway {
    async fn remove_session(&self, session_id: &SessionId) {
        sessions::Entity::delete_by_id(session_id.clone())
            .exec(&*self.db)
            .await
            .unwrap();
        cmd("DEL")
            .arg(session_id.to_string())
            .query_async::<_, ()>(&mut self.cache_redis_pool.get().await.unwrap())
            .await
            .ok();
    }
    
    async fn remove_user_sessions(&self, user_id: &UserId) {
        let session_models = sessions::Entity::find()
            .filter(Expr::col(sessions::Column::UserId).eq(user_id.to_string()))
            .all(&*self.db)
            .await
            .unwrap();
        
        sessions::Entity::delete_many()
            .filter(Expr::col(sessions::Column::UserId).eq(user_id.to_string()))
            .exec(&*self.db)
            .await
            .unwrap();
        
        cmd("DEL")
            .arg(session_models.iter().map(|s| s.token_hash.to_string()).collect::<Vec<String>>())
            .query_async::<_, ()>(&mut self.cache_redis_pool.get().await.unwrap())
            .await
            .ok();
    }
}

impl SessionGatewayTrait for SessionGateway {}

fn map_session_model_to_domain(model: sessions::Model) -> Session {
    Session {
        id: model.id,
        token_hash: model.token_hash,
        user_id: model.user_id,
        ip: model.ip,
        client: model.client,
        os: model.os,
        device: model.device,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
