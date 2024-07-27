use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::object::Object;

pub struct ObjectService { }

impl ObjectService {

    pub fn create_box(
        &self, 
        name: String,
        path: String,
        hash: String,
        size: u64,
        content_type: String,
        metadata: HashMap<String, String>,
    ) -> Object {
        Object {
            id: Uuid::new_v4(),
            name,
            path,
            hash,
            size,
            content_type,
            metadata,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update_box(
        &self,
        object: Object,
        new_name: String,
        new_path: String,
        new_metadata: HashMap<String, String>,
    ) -> Object {
        Object {
            name: new_name,
            path: new_path,
            metadata: new_metadata,
            updated_at: Some(Utc::now()),
            ..object
        }
    }
}
