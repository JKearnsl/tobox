use actix_web::HttpRequest;

use crate::adapters::auth::token::{IdTokenProvider, TokenProcessor};
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;

pub fn make_token_provider(
    req: &HttpRequest,
    token_processor: &TokenProcessor
) -> Result<Box<dyn IdProvider>, ApplicationError> {
    let token = req.cookie("token").map(|cookie| cookie.value().to_string());
    match IdTokenProvider::new(token, token_processor) {
        Ok(provider) => Ok(Box::new(provider)),
        Err(error) => Err(ApplicationError::Unauthorized(ErrorContent::from(error)))
    }
}
