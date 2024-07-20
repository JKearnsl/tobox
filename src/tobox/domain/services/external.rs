use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::service::{Service, ServiceTextId};

pub struct ExternalService {}

impl ExternalService {
    pub fn create_service(
        &self,
        text_id: ServiceTextId,
        title: String,
        description: Option<String>,
    ) -> Service {
        Service {
            id: Uuid::new_v4(),
            text_id,
            title,
            description,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
    
    pub fn update_service(
        &self,
        service: Service,
        new_title: String,
        new_description: Option<String>,
    ) -> Service {
        Service {
            title: new_title,
            description: new_description,
            updated_at: Some(Utc::now()),
            ..service
        }
    }
}
