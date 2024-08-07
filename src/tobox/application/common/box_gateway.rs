use async_trait::async_trait;

use crate::domain::models::r#box::{Box as BoxDomain, BoxId};

#[async_trait]
pub trait BoxReader {
    async fn get_box(&self, box_id: &BoxId) -> Option<BoxDomain>;
    async fn get_boxes(&self) -> Vec<BoxDomain>;
    async fn get_boxes_range(&self, limit: &u64, offset: &u64) -> Vec<BoxDomain>;
}

#[async_trait]
pub trait BoxWriter {
    async fn save_box(&self, data: &BoxDomain);
}

#[async_trait]
pub trait BoxRemover {
    async fn remove_box(&self, box_id: &BoxId);
}

pub trait BoxGateway: BoxReader + BoxWriter + BoxRemover {}