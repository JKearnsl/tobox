use async_trait::async_trait;

use crate::domain::models::object::{Object as ObjectDomain, ObjectId};

#[async_trait]
pub trait ObjectReader {
    async fn get_object_by_id(&self, object_id: &ObjectId) -> Option<ObjectDomain>;
    async fn get_objects(&self) -> Vec<ObjectDomain>;
    async fn get_objects_range(&self, limit: &u64, offset: &u64) -> Vec<ObjectDomain>;
    async fn get_object_by_name_and_path(&self, name: &String, path: &String) -> Option<ObjectDomain>;
}

#[async_trait]
pub trait ObjectWriter {
    async fn save_object(&self, data: &ObjectDomain);
}

#[async_trait]
pub trait ObjectRemover {
    async fn remove_object(&self, object_id: &ObjectId);
}

pub trait ObjectGateway: ObjectReader + ObjectWriter + ObjectRemover {}
