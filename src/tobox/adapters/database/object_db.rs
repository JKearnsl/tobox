use core::option::Option;
use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::adapters::database::pool::DbPool;
use crate::application::common::object_gateway::{
    ObjectGateway as ObjectGatewayTrait,
    ObjectReader,
    ObjectRemover,
    ObjectWriter
};
use crate::domain::models::object::{Object as ObjectDomain, ObjectId};
use crate::adapters::database::models::objects::Object;

pub struct ObjectGateway{
    db: DbPool,
}

impl ObjectGateway {
    pub fn new(db: DbPool) -> Self {
        ObjectGateway {
            db,
        }
    }
}

#[async_trait]
impl ObjectReader for ObjectGateway {
    async fn get_object(&self, object_id: &ObjectId) -> Option<ObjectDomain> {
        let row: Option<Object> = sqlx::query_as("SELECT * FROM objects WHERE id = $1")
            .bind(object_id)
            .fetch_optional(&self.db).await?;
        
        match row {
            None => None,
            Some(row) => Some(map_object_to_domain(row))
        }
    }

    async fn get_objects(&self) -> Vec<ObjectDomain> {  // todo: ids?
        let rows: Vec<Object> = sqlx::query_as("SELECT * FROM objects").fetch_all(&self.db).await?;

        rows.into_iter().map(|row| map_object_to_domain(row)).collect()
    }

    async fn get_objects_range(&self, limit: &u64, offset: &u64) -> Vec<ObjectDomain> {
        let rows: Vec<Object> = sqlx::query_as("SELECT * FROM objects LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db).await.unwrap();

        rows.into_iter().map(|row| map_object_to_domain(row)).collect()
    }
}

#[async_trait]
impl ObjectWriter for ObjectGateway {
    async fn save_object(&self, object: &ObjectDomain) {
        match sqlx::query_as("SELECT id FROM objects WHERE id = $1")
            .bind(&object.id)
            .fetch_optional(&self.db).await.unwrap() {
            Some(_) => {
                sqlx::query(
                    "UPDATE objects SET
                        name = $2,
                        hash = $3,
                        size = $4,
                        content_type = $5,
                        metadata = $6,
                        box_id = $7,
                        created_at = $8
                        updated_at = $9
                    WHERE id = $1"
                )
                    .bind(&object.id)
                    .bind(&object.name)
                    .bind(&object.hash)
                    .bind(&object.size)
                    .bind(&object.content_type)
                    .bind(hashmap_to_bytes(object.metadata.clone()))
                    .bind(&object.box_id)
                    .bind(&object.created_at)
                    .bind(&object.updated_at)
                    .execute(&self.db).await.unwrap();
            },
            None => {
                sqlx::query(
                    "INSERT INTO objects (
                        id, 
                        name, 
                        hash, 
                        size, 
                        content_type, 
                        metadata, 
                        box_id, 
                        created_at, 
                        updated_at
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
                )
                    .bind(&object.id)
                    .bind(&object.name)
                    .bind(&object.hash)
                    .bind(&object.size)
                    .bind(&object.content_type)
                    .bind(hashmap_to_bytes(object.metadata.clone()))
                    .bind(&object.box_id)
                    .bind(&object.created_at)
                    .bind(&object.updated_at)
                    .execute(&self.db).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl ObjectRemover for ObjectGateway {
    async fn remove_object(&self, object_id: &ObjectId) {
        sqlx::query("DELETE FROM objects WHERE id = $1")
            .bind(object_id)
            .execute(&self.db).await.unwrap();
    }
}


fn map_object_to_domain(object: Object) -> ObjectDomain {
    ObjectDomain {
        id: object.id,
        name: object.name,
        hash: object.hash,
        size: object.size,
        content_type: object.content_type,
        metadata: bytes_to_hashmap(object.metadata),
        box_id: object.box_id,
        created_at: object.created_at,
        updated_at: object.updated_at,
    }
}

#[derive(Serialize, Deserialize)]
struct MapStub {
    map: HashMap<String, String>,
}

fn hashmap_to_bytes(map: HashMap<String, String>) -> Vec<u8> {
    let my_struct = MapStub { map };
    bincode::serialize(&my_struct).unwrap()
}

fn bytes_to_hashmap(bytes: Vec<u8>) -> HashMap<String, String> {
    let my_struct: MapStub = bincode::deserialize(&bytes).unwrap();
    my_struct.map
}

impl ObjectGatewayTrait for ObjectGateway {}
