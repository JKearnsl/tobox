use std::fmt::{Display, Formatter};

use actix_web::{error, HttpResponse, Result};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde_json::{json, Value};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // stub implementation
    }
}

impl ApplicationError {
    fn error_rest_content(&self) -> (StatusCode, ErrorContent) {
        match *self {
            ApplicationError::InvalidData(ref content) => (StatusCode::BAD_REQUEST, content.clone()),
            ApplicationError::NotFound(ref content) => (StatusCode::NOT_FOUND, content.clone()),
            ApplicationError::Conflict(ref content) => (StatusCode::CONFLICT, content.clone()),
            ApplicationError::Forbidden(ref content) => (StatusCode::FORBIDDEN, content.clone()),
            ApplicationError::Unauthorized(ref content) => (StatusCode::UNAUTHORIZED, content.clone()),
        }
    }
    
    pub fn as_json(&self) -> Value {
        match self.error_rest_content().1 {
            ErrorContent::Message(msg) => json!({
                "error": msg
            }),
            ErrorContent::Map(map) => json!({
                "errors": map.iter().map(|(field, message)| {
                    json!({
                        "field": field,
                        "message": message
                    })
                }).collect::<Vec<_>>()
            }),
        }
    }
}

impl error::ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        self.error_rest_content().0
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self.as_json()).unwrap())
    }
}


pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&json!({
            "error": "Запрашиваемый ресурс не найден"
        })).unwrap()))
}