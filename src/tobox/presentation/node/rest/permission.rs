use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Result, web};
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::permission::get_range::GetPermissionRangeDTO;
use crate::application::permission::link::LinkRolePermissionDTO;
use crate::application::permission::unlink::UnlinkRolePermissionDTO;
use crate::application::permission::update::UpdatePermissionDTO;
use crate::domain::models::role::RoleId;
use crate::domain::models::user::UserId;
use crate::presentation::node::id_provider::make_id_provider_from_request;
use crate::presentation::node::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/permission")
            .service(get_permissions)
            .service(update_permission)
            .service(link_role_permission)
            .service(unlink_role_permission)
    );
}

#[derive(Debug, Deserialize)]
struct PermissionsQuery {
    role_id: Option<RoleId>,
    user_id: Option<UserId>,
    page: Option<u64>,
    per_page: Option<u64>
}

#[get("")]
async fn get_permissions(
    data: web::Query<PermissionsQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {

    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );

    if let Some(role_id) = &data.role_id {
        return Ok(HttpResponse::Ok().json(
            ioc.get_role_permissions(id_provider).execute(role_id.clone()).await?
        ))
    } else if let Some(user_id) = &data.user_id {
        return Ok(HttpResponse::Ok().json(
            ioc.get_user_permissions(id_provider).execute(user_id.clone()).await?
        ))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_permission_range(id_provider).execute(
            GetPermissionRangeDTO {
                page: page.clone(),
                per_page: per_page.clone()
            }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    }
    Err(ApplicationError::InvalidData(ErrorContent::Message("Invalid query".to_string())))
}

#[put("")]
async fn update_permission(
    data: web::Json<UpdatePermissionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_permission(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[post("link")]
async fn link_role_permission(
    data: web::Json<LinkRolePermissionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.link_role_permission(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[delete("link")]
async fn unlink_role_permission(
    data: web::Json<UnlinkRolePermissionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.unlink_role_permission(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
