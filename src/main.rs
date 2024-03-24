use actix_web::{
    App,
    HttpServer,
    middleware::Logger,
    web
};

mod routes;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().service(web::scope("/api/v1")
            .service(web::scope("/panel").configure(routes::auth::router))
        )
            .wrap(Logger::new("[%s] [%{r}a] %U"))
    }).bind(("127.0.0.1", 8080))?.run().await
}
