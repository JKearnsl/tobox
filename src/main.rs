use actix_web::{
    App,
    HttpServer,
    middleware::Logger,
    web
};

mod routes;
mod models;
mod config;

struct AppState {
    config: config::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = config::Config::new().unwrap();
    let host = config.host.clone();
    let port = config.port.clone();

    log::info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(AppState {
            config: config.clone(),
        })).service(web::scope("/api/v1")
            .configure(routes::api::auth::router)
        )
            .wrap(Logger::new("[%s] [%{r}a] %U"))
    }).bind((host, port))?.run().await
}
