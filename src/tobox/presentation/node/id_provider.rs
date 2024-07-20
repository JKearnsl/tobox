use actix_web::HttpRequest;

use crate::adapters::auth::header::{HeaderPayload, IdHeaderProvider};
use crate::application::common::id_provider::IdProvider;
use crate::domain::models::service::ServiceTextId;

pub fn make_id_provider(
    service_name: &ServiceTextId,
    payload: Option<HeaderPayload>,
    user_agent: Option<String>,
    ip: &str
) -> Box<dyn IdProvider> {
    
    Box::new(IdHeaderProvider::new(
        service_name,
        payload,
        user_agent.unwrap_or("Unknown".to_string()),
        ip
    ))
}

pub fn make_id_provider_from_request(
    service_name: &ServiceTextId,
    is_intermediate: bool,
    req: &HttpRequest
) -> Box<dyn IdProvider> {
    let headers = req.headers();
    
    let payload_raw = headers.get("payload").map(|value| {
        value.to_str().unwrap().to_string()
    });
    let user_agent = headers.get("user-agent").map(|value| {
        value.to_str().unwrap().to_string()
    });
    
    let remote_addr = {
        if is_intermediate {
            req.connection_info().realip_remote_addr().unwrap().to_string()
        } else {
            req.connection_info().peer_addr().unwrap().to_string()
        }
    };

    let payload: Option<HeaderPayload> = match payload_raw {
        Some(payload_raw) => serde_json::from_str(&payload_raw).ok(),
        None => None
    };
    
    make_id_provider(
        service_name,
        payload,
        user_agent,
        remote_addr.as_str()
    )
}