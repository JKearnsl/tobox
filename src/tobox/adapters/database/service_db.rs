use async_trait::async_trait;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;

use crate::adapters::database::models::services;
use crate::application::common::service_gateway::{
    ServiceGateway as ServiceGatewayTrait,
    ServiceReader,
    ServiceRemover,
    ServiceWriter
};
use crate::domain::models::service::{Service, ServiceId, ServiceTextId};

pub struct ServiceGateway {
    db: Box<DbConn>,
}

impl ServiceGateway {
    pub fn new(
        db: Box<DbConn>
    ) -> Self {
        ServiceGateway {
            db
        }
    }
}

#[async_trait]
impl ServiceReader for ServiceGateway {
    async fn get_service_by_id(&self, service_id: &ServiceId) -> Option<Service> {
        services::Entity::find_by_id(service_id.clone())
            .one(&*self.db)
            .await
            .unwrap()
            .map(map_service_model_to_domain)
    }

    async fn get_services_by_text_id(&self, text_id: &ServiceTextId) -> Option<Service> {
        services::Entity::find()
            .filter(services::Column::TextId.eq(text_id))
            .one(&*self.db)
            .await
            .unwrap()
            .map(map_service_model_to_domain)
    }
    
    async fn get_services(&self, limit: &u64, offset: &u64) -> Vec<Service> {
        services::Entity::find()
            .limit(*limit)
            .offset(*offset)
            .all(&*self.db)
            .await
            .unwrap()
            .into_iter()
            .map(map_service_model_to_domain)
            .collect()
    }
}

#[async_trait]
impl ServiceWriter for ServiceGateway {
    async fn save_service(&self, data: &Service) {
        let model = services::ActiveModel {
            id: Set(data.id.clone()),
            text_id: Set(data.text_id.clone()),
            title: Set(data.title.clone()),
            description: Set(data.description.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at),
        };

        match services::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                services::Entity::update(model).exec(&*self.db).await.unwrap();
            },
            None => {
                services::Entity::insert(model).exec(&*self.db).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl ServiceRemover for ServiceGateway {
    async fn remove_service(&self, service_id: &ServiceId) {
        services::Entity::delete_by_id(service_id.clone())
            .exec(&*self.db)
            .await
            .unwrap();
    }
}

impl ServiceGatewayTrait for ServiceGateway {}

fn map_service_model_to_domain(model: services::Model) -> Service {
    Service {
        id: model.id,
        text_id: model.text_id,
        title: model.title,
        description: model.description,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
