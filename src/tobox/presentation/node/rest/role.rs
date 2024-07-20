use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Result, web};
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::role::create::CreateRoleDTO;
use crate::application::role::delete::DeleteRoleDTO;
use crate::application::role::get_by_id::GetRoleByIdDTO;
use crate::application::role::get_by_ids::GetRolesByIdsDTO;
use crate::application::role::get_by_user::GetUserRolesDTO;
use crate::application::role::get_range::RoleRangeDTO;
use crate::application::role::link::LinkRoleUserDTO;
use crate::application::role::unlink::UnlinkRoleUserDTO;
use crate::application::role::update::UpdateRoleDTO;
use crate::domain::models::role::RoleId;
use crate::domain::models::user::UserId;
use crate::presentation::node::id_provider::make_id_provider_from_request;
use crate::presentation::node::interactor_factory::InteractorFactory;
use crate::presentation::panel::deserializers::deserialize_uuid_list;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(create_role)
            .service(get_roles)
            .service(update_role)
            .service(delete_role)
            .service(set_default_role)
            .service(get_default_role)
            .service(link_role_user)
            .service(unlink_role_user)
    );
}

#[post("")]
async fn create_role(
    data: web::Json<CreateRoleDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.create_role(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}


#[derive(Debug, Deserialize)]
struct RolesQuery {
    id: Option<RoleId>,
    #[serde(deserialize_with = "deserialize_uuid_list", default)]
    ids: Option<Vec<RoleId>>,
    user_id: Option<UserId>,
    page: Option<u64>,
    per_page: Option<u64>
}

#[get("")]
async fn get_roles(
    data: web::Query<RolesQuery>,
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
        let data = ioc.get_role_by_id(id_provider).execute(
            GetRoleByIdDTO { id: id.clone() }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let Some(ids) = &data.ids {
        let data = ioc.get_roles_by_ids(id_provider).execute(
            GetRolesByIdsDTO { ids: ids.clone(), }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let Some(user_id) = &data.user_id {
        let data = ioc.get_role_by_user(id_provider).execute(
            GetUserRolesDTO { user_id: user_id.clone() }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_role_range(id_provider).execute(
            RoleRangeDTO {
                page: page.clone(),
                per_page: per_page.clone()
            }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    }
    Err(ApplicationError::InvalidData(ErrorContent::Message("Invalid query".to_string())))
}

#[put("")]
async fn update_role(
    data: web::Json<UpdateRoleDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_role(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[delete("")]
async fn delete_role(
    data: web::Json<DeleteRoleDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.delete_role(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Deserialize)]
struct DefaultRoleQuery {
    id: RoleId,
}

#[post("default")]
async fn set_default_role(
    data: web::Json<DefaultRoleQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.set_default_role(id_provider).execute(data.id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("default")]
async fn get_default_role(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_default_role(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}


#[post("link")]
async fn link_role_user(
    data: web::Json<LinkRoleUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.link_role_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[delete("link")]
async fn unlink_role_user(
    data: web::Json<UnlinkRoleUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.unlink_role_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
