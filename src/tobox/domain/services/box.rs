use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::r#box::Box;

pub struct BoxService { }

impl BoxService {

    pub fn create_box(&self, name: String) -> Box {
        Box {
            id: Uuid::new_v4(),
            name,
            created_at: Utc::now(),
        }
    }

    pub fn update_box(&self, r#box: Box, new_name: String) -> Box {
        Box {
            name: new_name,
            ..r#box
        }
    }
}
