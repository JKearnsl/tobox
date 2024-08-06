use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::pool::DbPool;
use crate::application::common::user_gateway::{
    UserGateway as UserGatewayTrait, 
    UserReader, 
    UserRemover, 
    UserWriter
};
use crate::domain::models::user::{User as UserDomain, UserId};
use crate::adapters::database::models::users::User;

pub struct UserGateway{
    db: DbPool,
}

impl UserGateway {
    pub fn new(db: DbPool) -> Self {
        UserGateway {
            db,
        }
    }
}

#[async_trait]
impl UserReader for UserGateway {
    async fn get_user(&self, user_id: &UserId) -> Option<UserDomain> {
        let row: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db).await?;
        
        match row {
            None => None,
            Some(row) => Some(map_user_model_to_domain(row))
        }
    }

    async fn get_users(&self, user_ids: &Vec<UserId>) -> Option<Vec<UserDomain>> {
        let rows: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE id = ANY($1)")
            .bind(user_ids)
            .fetch_all(&self.db).await?;
        
        if rows.is_empty() {
            None
        } else {
            Some(rows.into_iter().map(|row| map_user_model_to_domain(row)).collect())
        }
    }

    async fn get_users_range(&self, limit: &u64, offset: &u64) -> Vec<UserDomain> {
        let rows: Vec<User> = sqlx::query_as("SELECT * FROM users LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db).await.unwrap();
        
        rows.into_iter().map(|row| map_user_model_to_domain(row)).collect()
    }

    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Option<UserDomain> {
        let row: Option<User> = sqlx::query_as(
            "SELECT * FROM users WHERE username = $1 COLLATE NOCASE"
        ).bind(username).fetch_optional(&self.db).await?;
        
        match row {
            None => None,
            Some(row) => Some(map_user_model_to_domain(row))
        }
    }
}

#[async_trait]
impl UserWriter for UserGateway {
    async fn save_user(&self, user: &UserDomain) {
        match sqlx::query_as("SELECT id FROM users WHERE id = $1")
            .bind(&user.id)
            .fetch_optional(&self.db).await.unwrap() {
            Some(_) => {
                sqlx::query(
                    "UPDATE users SET username = $2, hashed_password = $3, created_at = $4 \
                    WHERE id = $1"
                )
                    .bind(&user.id)
                    .bind(&user.username)
                    .bind(&user.hashed_password)
                    .bind(&user.created_at)
                    .execute(&self.db).await.unwrap();
            },
            None => {
                sqlx::query(
                    "INSERT INTO users (id, username, hashed_password, created_at) \
                    VALUES ($1, $2, $3, $4)"
                )
                    .bind(&user.id)
                    .bind(&user.username)
                    .bind(&user.hashed_password)
                    .bind(&user.created_at)
                    .execute(&self.db).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl UserRemover for UserGateway {
    async fn remove_user(&self, user_id: &UserId) {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.db).await.unwrap();
    }
}

fn map_user_model_to_domain(user: User) -> UserDomain {
    UserDomain {
        id: user.id,
        username: user.username,
        hashed_password: user.hashed_password,
        created_at: user.created_at,
    }
}

impl UserGatewayTrait for UserGateway {}
