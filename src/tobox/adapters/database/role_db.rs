use core::option::Option;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cached::{Cached, TimedCache};
use sea_orm::{DbBackend, DbConn, EntityTrait, QueryFilter, QuerySelect, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Expr};
use sea_orm::sea_query::extension::postgres::PgExpr;

use crate::adapters::database::models::{default_role, role_user, roles};
use crate::application::common::role_gateway::{RoleGateway as RoleGatewayTrait, RoleLinker, RoleReader, RoleRemover, RoleWriter};
use crate::domain::models::role::{Role as RoleDomain, RoleId};
use crate::domain::models::user::UserId;

pub struct RoleGateway{
    pub db: Box<DbConn>,
    cache_role_by_id: Arc<Mutex<TimedCache<RoleId, roles::Model>>>,
}

impl RoleGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        RoleGateway {
            db,
            cache_role_by_id: Arc::new(Mutex::new(TimedCache::with_lifespan(3))),

        }
    }
}

#[async_trait]
impl RoleReader for RoleGateway {
    async fn get_role(&self, role_id: &RoleId) -> Option<RoleDomain> {

        let cached_value = self.cache_role_by_id.lock().unwrap().cache_get(role_id).cloned();
        if cached_value.is_some() {
            return match cached_value {
                Some(value) => Some(map_role_model_to_domain(value.clone())),
                None => None
            }
        }

        match roles::Entity::find_by_id(role_id.clone()).one(&*self.db).await.unwrap() {
            Some(role) => {
                self.cache_role_by_id.lock().unwrap().cache_set(role_id.clone(), role.clone());
                Option::from(map_role_model_to_domain(role))
            }
            None => None
        }
    }

    async fn get_roles_by_ids(&self, role_ids: &Vec<RoleId>) -> Option<Vec<RoleDomain>> {
        let roles: Vec<roles::Model> = roles::Entity::find().filter(
            {
                let mut condition = Condition::any();
                for id in role_ids {
                    condition = condition.add(Expr::col(roles::Column::Id).eq(*id));
                }
                condition
            }
            )
            .all(&*self.db)
            .await
            .unwrap();

        if roles.len() != role_ids.len() {
            return None
        }

        Option::from(
            roles.iter()
                .map(|role| map_role_model_to_domain(role.clone()))
                .collect::<Vec<_>>()
        )
    }

    async fn get_roles_range(&self, limit: &u64, offset: &u64) -> Vec<RoleDomain> {
        let roles: Vec<roles::Model> = roles::Entity::find()
            .offset(*offset)
            .limit(*limit)
            .all(&*self.db)
            .await
            .unwrap();
        roles.iter().map(|role| map_role_model_to_domain(role.clone())).collect()
    }

    async fn get_user_roles(&self, user_id: &UserId) -> Vec<RoleDomain> {
        let raw_sql = r#"
            SELECT
                role.*
            FROM
                role
            JOIN
                role_user ON role.id = role_user.role_id
            WHERE
                role_user.user_id = $1;
        "#;

        let roles: Vec<roles::Model> = roles::Entity::find().from_raw_sql(
            Statement::from_sql_and_values(
                DbBackend::Postgres,
                raw_sql,
                vec![user_id.clone().into()]
            )
        )
            .all(&*self.db)
            .await.unwrap();

        roles.into_iter().map(map_role_model_to_domain).collect()
    }
    
    async fn get_role_by_title_not_sensitive(&self, title: &String) -> Option<RoleDomain> {
        let role: Option<roles::Model> = roles::Entity::find().filter(
                Expr::col(roles::Column::Title).ilike(title)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match role {
            Some(role) => Some(map_role_model_to_domain(role)),
            None => None
        }
    }

    async fn get_default_role(&self) -> Option<RoleDomain> {
        default_role::Entity::find().find_also_related(roles::Entity).one(&*self.db).await.unwrap()
            .map(
                |(_, role)| map_role_model_to_domain(role.unwrap())
            )
    }
}

#[async_trait]
impl RoleWriter for RoleGateway {
    async fn save_role(&self, data: &RoleDomain) {
        let role_model = roles::ActiveModel {
            id: Set(data.id),
            title: Set(data.title.clone()),
            description: Set(data.description.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at.clone())
        };

        match roles::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                roles::Entity::update(role_model).exec(&*self.db).await.unwrap();
            }
            None => {
                roles::Entity::insert(role_model).exec(&*self.db).await.unwrap();
            }
        }
    }

    async fn set_default_role(&self, role_id: &RoleId) {
        default_role::Entity::delete_many().exec(&*self.db).await.unwrap();
        default_role::Entity::insert(default_role::ActiveModel {
            id: Set(role_id.clone()),
        }).exec(&*self.db).await.unwrap();
    }
}

#[async_trait]
impl RoleLinker for RoleGateway {
    async fn link_role_to_user(&self, role_id: &RoleId, user_id: &UserId) {
        role_user::Entity::insert(role_user::ActiveModel {
            role_id: Set(role_id.clone()),
            user_id: Set(user_id.clone())
        }).exec(&*self.db).await.unwrap();
    }

    async fn unlink_role_from_user(&self, role_id: &RoleId, user_id: &UserId) {
        role_user::Entity::delete_many()
            .filter(
                Expr::col(role_user::Column::RoleId).eq(role_id.clone())
                    .and(Expr::col(role_user::Column::UserId).eq(user_id.clone()))
            )
            .exec(&*self.db)
            .await
            .unwrap();
    }

    async fn is_role_linked_to_user(&self, role_id: &RoleId, user_id: &UserId) -> bool {
        role_user::Entity::find()
            .filter(
                Expr::col(role_user::Column::RoleId).eq(role_id.clone())
                    .and(Expr::col(role_user::Column::UserId).eq(user_id.clone()))
            )
            .one(&*self.db)
            .await
            .unwrap()
            .is_some()
    }
}

#[async_trait]
impl RoleRemover for RoleGateway {
    async fn remove_role(&self, role_id: &RoleId) {
        roles::Entity::delete_by_id(role_id.clone())
            .exec(&*self.db)
            .await
            .unwrap();
        
    }
}

fn map_role_model_to_domain(role: roles::Model) -> RoleDomain {
    RoleDomain {
        id: role.id,
        title: role.title,
        description: role.description,
        created_at: role.created_at,
        updated_at: role.updated_at
    }
}


impl RoleGatewayTrait for RoleGateway {}
