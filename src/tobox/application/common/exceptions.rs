use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum ErrorContent {
    Message(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Serialize, Clone)]
pub enum ApplicationError {
    InvalidData(ErrorContent),
    NotFound(ErrorContent),
    Conflict(ErrorContent),
    Unauthorized(ErrorContent),
    Forbidden(ErrorContent),
}
