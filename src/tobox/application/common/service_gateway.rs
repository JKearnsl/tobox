use async_trait::async_trait;

use crate::domain::models::service::{Service, ServiceId, ServiceTextId};

#[async_trait]
pub trait ServiceReader {
    async fn get_service_by_id(&self, service_id: &ServiceId) -> Option<Service>;
    async fn get_services(&self, limit: &u64, offset: &u64) -> Vec<Service>;
    async fn get_services_by_text_id(&self, text_id: &ServiceTextId) -> Option<Service>;
}

#[async_trait]
pub trait ServiceWriter {
    async fn save_service(&self, data: &Service);
}

#[async_trait]
pub trait ServiceRemover {
    async fn remove_service(&self, service_id: &ServiceId);
}


pub trait ServiceGateway: ServiceReader + ServiceWriter + ServiceRemover + Send + Sync {}