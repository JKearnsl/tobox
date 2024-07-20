//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_permissions::Entity")]
    RolePermissions,
    #[sea_orm(has_many = "super::role_user::Entity")]
    RoleUser,
}

impl Related<super::role_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermissions.def()
    }
}

impl Related<super::role_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleUser.def()
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permissions::Relation::Permissions.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_permissions::Relation::Roles.def().rev())
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_user::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_user::Relation::Roles.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
