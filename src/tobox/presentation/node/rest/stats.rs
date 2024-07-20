use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;

use crate::presentation::panel::server::AppConfigProvider;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stats")
            .service(stats)
    );
}


#[get("")]
async fn stats(
    app_config_provider: web::Data<AppConfigProvider>
) -> impl Responder {
    HttpResponse::Ok().json(json!(
        {
            "service": app_config_provider.service_name,
            "build": app_config_provider.build,
            "branch": app_config_provider.branch,
            "version": app_config_provider.version,
        }
    ))
}

