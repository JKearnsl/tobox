use std::collections::HashMap;
use chrono::Utc;
use crate::domain::id_generator::generate_id;
use crate::domain::models::object::{Object, ObjectId};
use crate::domain::models::r#box::BoxId;

pub struct ObjectService { }

impl ObjectService {
    
    pub fn generate_object_id(&self) -> ObjectId {
        generate_id(25)
    }

    pub fn create_object(
        &self, 
        id: ObjectId,
        name: Option<String>,
        hash: String,
        size: u64,
        content_type: String,
        metadata: HashMap<String, String>,
        box_id: BoxId
    ) -> Object {
        Object {
            id,
            name,
            hash,
            size,
            content_type,
            metadata,
            box_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update_object(
        &self,
        object: Object,
        new_name: Option<String>,
        new_metadata: HashMap<String, String>,
    ) -> Object {
        Object {
            name: new_name,
            metadata: new_metadata,
            updated_at: Some(Utc::now()),
            ..object
        }
    }
}
