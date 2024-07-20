use actix_web::{get, HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::session::get_access_log::GetAccessLogDTO;
use crate::application::session::get_access_log_self::GetAccessLogSelfDTO;
use crate::domain::models::user::UserId;
use crate::presentation::node::id_provider::make_id_provider_from_request;
use crate::presentation::node::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/access_log")
            .service(access_log)
    );
}


#[derive(Debug, Deserialize)]
struct QueryParams {
    page: u64,
    per_page: u64,
    user_id: Option<UserId>
}

#[get("")]
async fn access_log(
    query: web::Query<QueryParams>,
    app_config_provider: web::Data<AppConfigProvider>,
    ioc: web::Data<dyn InteractorFactory>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    
    match query.user_id {
        Some(user_id) => {
            let data = ioc.get_access_log(id_provider).execute(
                GetAccessLogDTO {
                    user_id,
                    page: query.page,
                    per_page: query.per_page
                }
            ).await?;
            Ok(HttpResponse::Ok().json(data))
        },
        None => {
            let data = ioc.get_access_log_self(id_provider).execute(
                GetAccessLogSelfDTO {
                    page: query.page,
                    per_page: query.per_page
                }
            ).await?;
            
            Ok(HttpResponse::Ok().json(data))
        }
    }
}
