use std::collections::HashMap;

use serde::Serialize;
use crate::domain::exceptions::DomainError;

#[derive(Debug, Serialize, Clone)]
pub enum ErrorContent<T: Into<String> = String> {
    Message(T),
    Map(HashMap<String, String>),
}

impl From<DomainError> for ErrorContent<String> {
    fn from(error: DomainError) -> Self {
        ErrorContent::Message(error.to_string())
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
