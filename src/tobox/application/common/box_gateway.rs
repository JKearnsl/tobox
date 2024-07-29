use async_trait::async_trait;

use crate::domain::models::r#box::{Box as BoxDomain, BoxId};

#[async_trait]
pub trait BoxReader {
    async fn get_box_by_id(&self, box_id: &BoxId) -> Option<BoxDomain>;
    async fn get_boxes(&self) -> Vec<BoxDomain>;
    async fn get_boxes_range(&self, limit: &u64, offset: &u64) -> Vec<BoxDomain>;
    async fn get_box_by_name_not_sensitive(&self, name: &String) -> Option<BoxDomain>;
}

#[async_trait]
pub trait BoxWriter {
    async fn save_box(&self, data: &BoxDomain);
}

pub trait BoxGateway: BoxReader + BoxWriter {}