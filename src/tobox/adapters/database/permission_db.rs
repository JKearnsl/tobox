use core::option::Option;

use async_trait::async_trait;
use sea_orm::{Condition, DbBackend, DbConn, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Expr, IntoCondition};

use crate::adapters::database::models::{permissions, role_permissions};
use crate::application::common::permission_gateway::{
    PermissionGateway as PermissionGatewayTrait,
    PermissionLinker,
    PermissionReader,
    PermissionRemover,
    PermissionWriter
};
use crate::domain::models::permission::{Permission as PermissionDomain, PermissionId, PermissionTextId};
use crate::domain::models::permission::Permission;
use crate::domain::models::role::RoleId;
use crate::domain::models::service::ServiceId;
use crate::domain::models::user::UserId;

pub struct PermissionGateway{
    pub db: Box<DbConn>,
}

impl PermissionGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        PermissionGateway {
            db,
        }
    }
}

#[async_trait]
impl PermissionReader for PermissionGateway {
    async fn get_permission_by_id(&self, permission_id: &PermissionId) -> Option<Permission> {
        match permissions::Entity::find_by_id(*permission_id)
            .one(&*self.db)
            .await.unwrap() {
            Some(permission) => Some(map_permission_model_to_domain(permission)),
            None => return None
        }
    }

    async fn get_permissions_by_service_id(&self, service_id: &ServiceId) -> Vec<Permission> {
        let permissions = permissions::Entity::find()
            .filter(Expr::col(permissions::Column::ServiceId).eq(*service_id))
            .all(&*self.db)
            .await.unwrap();
        permissions.into_iter().map(map_permission_model_to_domain).collect()
    }

    async fn get_permissions_by_ids(&self, permission_ids: &Vec<PermissionId>) -> Option<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .filter(
                {
                    let mut condition = Condition::any();
                    for id in permission_ids {
                        condition = condition.add(Expr::col(permissions::Column::Id).eq(*id));
                    }
                    condition
                }
            )
            .all(&*self.db)
            .await.unwrap();
        
        if permissions.len() != permission_ids.len() {
            return None
        }
        
        Some(permissions.into_iter().map(map_permission_model_to_domain).collect())
    }

    async fn get_permissions_by_text_ids(&self, permission_text_ids: &Vec<PermissionTextId>) -> Option<Vec<PermissionDomain>> {
        let permissions = permissions::Entity::find()
            .filter(
                {
                    let mut condition = Condition::any();
                    for text_id in permission_text_ids {
                        condition = condition.add(Expr::col(permissions::Column::TextId).eq(text_id));
                    }
                    condition
                }
            )
            .all(&*self.db)
            .await
            .unwrap();
        
        if permissions.len() != permission_text_ids.len() {
            return None
        }
        
        Some(permissions.into_iter().map(map_permission_model_to_domain).collect())
    }

    async fn get_permissions_list(&self, limit: &u64, offset: &u64) -> Vec<Permission> {
        let permissions = permissions::Entity::find()
            .limit(*limit)
            .offset(*offset)
            .all(&*self.db)
            .await.unwrap();
        permissions.into_iter().map(map_permission_model_to_domain).collect()
    }

    async fn get_role_permissions(&self, role_id: &RoleId) -> Vec<Permission> {
        let role_id = role_id.clone();
        let permissions = permissions::Entity::find()
            .join(
                JoinType::InnerJoin,
                permissions::Relation::RolePermissions
                    .def()
                    .rev()
                    .on_condition(move |_left, right| {
                        Expr::col((right, role_permissions::Column::RoleId)).eq(role_id)
                            .into_condition()
                    })
            )
            .all(&*self.db)
            .await.unwrap();
        permissions.into_iter().map(map_permission_model_to_domain).collect()
    }

    async fn get_user_permissions(&self, user_id: &UserId) -> Vec<PermissionDomain> {
        let raw_sql = r#"
            SELECT
                permission.*
            FROM
                permission
            JOIN
                role_permissions ON permission.permission_id = role_permissions.permission_id
            JOIN
                role_user ON role_permissions.role_id = role_user.role_id
            WHERE
                role_user.user_id = $1;
        "#;

        let permissions: Vec<permissions::Model> = permissions::Entity::find().from_raw_sql(
            Statement::from_sql_and_values(
                DbBackend::Postgres,
                raw_sql,
                vec![user_id.clone().into()]
            )
        )
            .all(&*self.db)
            .await.unwrap();

        permissions.into_iter().map(map_permission_model_to_domain).collect()
    }
}

#[async_trait]
impl PermissionWriter for PermissionGateway {
    async fn save_permission(&self, data: &Permission) {
        let model = permissions::ActiveModel {
            id: Set(data.id),
            text_id: Set(data.text_id.clone()),
            service_id: Set(data.service_id),
            title: Set(data.title.clone()),
            description: Set(data.description.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at),
        };
        
        match permissions::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                permissions::Entity::update(model).exec(&*self.db).await.unwrap();
            }
            None => {
                permissions::Entity::insert(model).exec(&*self.db).await.unwrap();
            }
        }
    }

    async fn save_permissions(&self, data: &Vec<Permission>) {
        let found_perms = permissions::Entity::find().filter(
            {
                let mut condition = Condition::any();
                for permission in data {
                    condition = condition.add(Expr::col(permissions::Column::Id).eq(permission.id));
                }
                condition
            }
        ).all(&*self.db).await.unwrap();
        
        let for_insert = data.iter().filter(|model| {
            !found_perms.iter().any(|perm| perm.id == model.id)
        }).collect::<Vec<_>>();
        
        if for_insert.len() != 0 {
            permissions::Entity::insert_many(
                for_insert.iter().map(|permission| {
                    map_permission_domain_to_model((**permission).clone())
                }).collect::<Vec<_>>()
            ).exec(&*self.db).await.unwrap();
        }
        
        let for_update = data.iter().filter(|model| {
            found_perms.iter().any(|perm| perm.id == model.id)
        }).collect::<Vec<_>>();
        
        // O(n) gather all
        for permission in for_update.iter() {
            permissions::Entity::update(map_permission_domain_to_model((**permission).clone()))
                .exec(&*self.db)
                .await.unwrap();
        }
    }
}

#[async_trait]
impl PermissionRemover for PermissionGateway {
    async fn remove_permission(&self, permission_id: PermissionId) {
        permissions::Entity::delete_by_id(permission_id).exec(&*self.db).await.unwrap();
    }
}

#[async_trait]
impl PermissionLinker for PermissionGateway {
    async fn link_permission_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        let model = role_permissions::ActiveModel {
            permission_id: Set(*permission_id),
            role_id: Set(*role_id),
        };
        role_permissions::Entity::insert(model).exec(&*self.db).await.unwrap();
    }

    async fn link_permissions_to_role(&self, role_id: &RoleId, permission_ids: &Vec<PermissionId>) {
        let models = permission_ids.iter().map(|permission_id| {
            role_permissions::ActiveModel {
                permission_id: Set(*permission_id),
                role_id: Set(*role_id),
            }
        }).collect::<Vec<_>>();
        role_permissions::Entity::insert_many(models).exec(&*self.db).await.unwrap();
    }

    async fn unlink_permission_from_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        role_permissions::Entity::delete_many()
            .filter(
                Expr::col(role_permissions::Column::RoleId).eq(*role_id)
                    .and(Expr::col(role_permissions::Column::PermissionId).eq(*permission_id))
            )
            .exec(&*self.db)
            .await
            .unwrap();
    }

    async fn is_permission_linked_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> bool {
        role_permissions::Entity::find()
            .filter(
                Expr::col(role_permissions::Column::RoleId).eq(*role_id)
                    .and(Expr::col(role_permissions::Column::PermissionId).eq(*permission_id))
            )
            .one(&*self.db)
            .await
            .unwrap()
            .is_some()
    }
}

fn map_permission_model_to_domain(permission: permissions::Model) -> PermissionDomain {
    PermissionDomain {
        id: permission.id,
        text_id: permission.text_id,
        service_id: permission.service_id,
        title: permission.title,
        description: permission.description,
        created_at: permission.created_at,
        updated_at: permission.updated_at,
    }
}

fn map_permission_domain_to_model(permission: PermissionDomain) -> permissions::ActiveModel {
    permissions::ActiveModel {
        id: Set(permission.id),
        text_id: Set(permission.text_id),
        service_id: Set(permission.service_id),
        title: Set(permission.title),
        description: Set(permission.description),
        created_at: Set(permission.created_at),
        updated_at: Set(permission.updated_at),
    }
}


impl PermissionGatewayTrait for PermissionGateway {}
