use actix_web::{delete, get, HttpRequest, HttpResponse, post, Result, web};
use actix_web::cookie::Cookie;
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::session::create::CreateSessionDTO;
use crate::application::session::delete::DeleteSessionDTO;
use crate::domain::models::session::SessionId;
use crate::domain::models::user::UserId;
use crate::presentation::node::id_provider::make_id_provider_from_request;
use crate::presentation::node::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sessions")
            .service(sessions_self)
            .service(create_session)
            .service(sessions_by)
            .service(delete_session)
            .service(delete_self_session)
    );
}

#[post("")]
async fn create_session(
    data: web::Json<CreateSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let (data, session_token) = ioc.create_session(id_provider).execute(
        data.into_inner()
    ).await?;
    
    let mut response = HttpResponse::Ok().json(data);
    response.add_cookie(
        &Cookie::build("session_token", session_token.to_string())
            .path("/")
            .http_only(true)
            .finish()
    ).unwrap();
    
    Ok(response)
}

#[delete("{id}")]
async fn delete_session(
    id: web::Path<DeleteSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.delete_session(id_provider).execute(
        id.into_inner()
    ).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[delete("self")]
async fn delete_self_session(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.delete_self_session(id_provider).execute(()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Deserialize)]
struct SessionsQueryParams {
    user_id: Option<UserId>,
    id: Option<SessionId>
}


#[get("")]
async fn sessions_by(
    data: web::Query<SessionsQueryParams>,
    app_config_provider: web::Data<AppConfigProvider>,
    ioc: web::Data<dyn InteractorFactory>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    
    if let Some(id) = data.id {
        let data = ioc.get_session_by_id(id_provider).execute(
            id
        ).await?;
        return Ok(HttpResponse::Ok().json(data));
    }
    
    if let Some(user_id) = data.user_id {
        let data = ioc.get_sessions_by_user_id(id_provider).execute(
            user_id
        ).await?;
        return Ok(HttpResponse::Ok().json(data));
    }
    
    Err(
        ApplicationError::InvalidData(
            ErrorContent::Message(
                "Необходимо указать user_id или id сессии".to_string()
            )
        )
    )
}

#[get("self")]
async fn sessions_self(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError>{
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_sessions_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}
