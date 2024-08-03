use chrono::Utc;
use crate::domain::id_generator::generate_id;
use crate::domain::models::r#box::Box;

pub struct BoxService { }

impl BoxService {

    pub fn create_box(&self) -> Box {
        Box {
            id: generate_id(16),
            created_at: Utc::now(),
        }
    }
}
