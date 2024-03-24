use actix_web::{post, Responder, web};

use crate::models;

pub(crate) fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/auth"));
    cfg.service(login);
}

#[post("signIn")]
async fn login(
    data: web::Json<models::auth::SignIn>
) -> impl Responder {
    format!("Hello {}!", data.email)
}
