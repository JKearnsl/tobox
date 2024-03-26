use actix_web::{get, post, Responder, web};

use crate::models;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("auth"));
    cfg.service(login);
    cfg.service(ping);
}

#[post("signIn")]
async fn login(
    data: web::Json<models::auth::SignIn>
) -> impl Responder {
    format!("Hello {}!", data.email)
}


#[get("ping")]
async fn ping() -> impl Responder {
    "pong"
}
