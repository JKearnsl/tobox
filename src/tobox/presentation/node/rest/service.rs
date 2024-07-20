use actix_web::{get, HttpRequest, HttpResponse, put, Result, web};
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::service::get_range::GetServiceRangeDTO;
use crate::application::service::update::UpdateServiceDTO;
use crate::domain::models::service::ServiceId;
use crate::presentation::node::id_provider::make_id_provider_from_request;
use crate::presentation::node::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/services")
            .service(get_services)
            .service(update_service)
    );
}

#[derive(Debug, Deserialize)]
struct ServicesQuery {
    id: Option<ServiceId>,
    page: Option<u64>,
    per_page: Option<u64>
}

#[get("")]
async fn get_services(
    data: web::Query<ServicesQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {

    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );

    if let Some(id) = &data.id {
        return Ok(HttpResponse::Ok().json(
            ioc.get_service(id_provider).execute(id.clone()).await?
        ))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_service_range(id_provider).execute(
            GetServiceRangeDTO {
                page: page.clone(),
                per_page: per_page.clone()
            }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    }
    Err(ApplicationError::InvalidData(ErrorContent::Message("Invalid query".to_string())))
}

#[put("")]
async fn update_service(
    data: web::Json<UpdateServiceDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_service(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}
