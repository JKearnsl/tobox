use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub content_type: String,
    pub size: u64,
    pub hash: String
}
