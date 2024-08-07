use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::pool::DbPool;
use crate::application::common::box_gateway::{
    BoxGateway as BoxGatewayTrait,
    BoxReader,
    BoxRemover,
    BoxWriter
};
use crate::domain::models::r#box::{Box as BoxDomain, BoxId};
use crate::adapters::database::models::boxes::Box as BoxModel;

pub struct BoxGateway{
    db: DbPool,
}

impl BoxGateway {
    pub fn new(db: DbPool) -> Self {
        BoxGateway {
            db,
        }
    }
}

#[async_trait]
impl BoxReader for BoxGateway {
    async fn get_box(&self, box_id: &BoxId) -> Option<BoxDomain> {
        let row: Option<BoxModel> = sqlx::query_as("SELECT * FROM boxes WHERE id = $1")
            .bind(box_id)
            .fetch_optional(&self.db).await?;
        
        match row {
            None => None,
            Some(row) => Some(BoxDomain{
                id: row.id,
                created_at: row.created_at,
            })
        }
    }

    async fn get_boxes(&self) -> Vec<BoxDomain> {  // todo: ids?
        let rows: Vec<BoxModel> = sqlx::query_as("SELECT * FROM boxes").fetch_all(&self.db).await?;

        rows.into_iter().map(|row| BoxDomain {
            id: row.id,
            created_at: row.created_at,
        }).collect()
    }

    async fn get_boxes_range(&self, limit: &u64, offset: &u64) -> Vec<BoxDomain> {
        let rows: Vec<BoxModel> = sqlx::query_as("SELECT * FROM boxes LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db).await.unwrap();

        rows.into_iter().map(|row| BoxDomain {
            id: row.id,
            created_at: row.created_at,
        }).collect()
    }
}

#[async_trait]
impl BoxWriter for BoxGateway {
    async fn save_box(&self, r#box: &BoxDomain) {
        match sqlx::query_as("SELECT id FROM boxes WHERE id = $1")
            .bind(&r#box.id)
            .fetch_optional(&self.db).await.unwrap() {
            Some(_) => {
                sqlx::query(
                    "UPDATE boxes SET created_at = $2 WHERE id = $1"
                )
                    .bind(&r#box.id)
                    .bind(&r#box.created_at)
                    .execute(&self.db).await.unwrap();
            },
            None => {
                sqlx::query(
                    "INSERT INTO boxes (id, created_at) VALUES ($1, $2)"
                )
                    .bind(&r#box.id)
                    .bind(&r#box.created_at)
                    .execute(&self.db).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl BoxRemover for BoxGateway {
    async fn remove_box(&self, box_id: &BoxId) {
        sqlx::query("DELETE FROM boxes WHERE id = $1")
            .bind(box_id)
            .execute(&self.db).await.unwrap();
    }
}

impl BoxGatewayTrait for BoxGateway {}
