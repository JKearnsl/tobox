use std::collections::HashMap;

use serde::Serialize;
use crate::domain::exceptions::DomainError;

#[derive(Debug, Serialize, Clone)]
pub enum ErrorContent {
    Message(String),
    Map(HashMap<String, String>),
}

impl From<DomainError> for ErrorContent {
    fn from(error: DomainError) -> Self {
        ErrorContent::Message(error.to_string())
    }
}

impl From<&str> for ErrorContent {
    fn from(s: &str) -> Self {
        ErrorContent::Message(s.to_string())
    }
}

impl From<String> for ErrorContent {
    fn from(s: String) -> Self {
        ErrorContent::Message(s)
    }
}

impl From<HashMap<String, String>> for ErrorContent {
    fn from(map: HashMap<String, String>) -> Self {
        ErrorContent::Map(map)
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum ApplicationError {
    InvalidData(ErrorContent),
    NotFound(ErrorContent),
    Conflict(ErrorContent),
    Unauthorized(ErrorContent),
    Forbidden(ErrorContent),
}
