use std::str::FromStr;
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
    let config = match config::Config::new("config.yaml") {
        Ok(config) => config,
        Err(error) => {
            env_logger::builder().filter_level(log::LevelFilter::Error).init();
            log::error!("Failed to load config: {}", error);
            std::process::exit(1);
        },
    };

    if let Some(log_level) = &config.log_level {
        env_logger::builder()
            .filter_level(log::LevelFilter::from_str(log_level).unwrap())
            .init();
    }

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
