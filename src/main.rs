use std::fs::File;
use std::io::BufReader;
use std::net::TcpListener;
use std::str::FromStr;
use std::thread;
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
    let tls = config.tls.clone();
    let workers = config.workers.unwrap_or(
        match thread::available_parallelism() {
            Ok(parallelism) => usize::from(parallelism),
            Err(_) => 1,
        }
    );

    let mut server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(AppState {
            config: config.clone(),
        })).service(web::scope("/api/v1")
            .configure(routes::api::auth::router)
        )
            .wrap(Logger::new("[%s] [%{r}a] %U"))
    });

    let listener = TcpListener::bind(format!("{}:{}", host, port)).map_err(
        |error| {
            log::error!("Failed to bind to address: {}", error.to_string());
            std::process::exit(1);
        }
    ).unwrap();

    if let Some(tls) = tls {
        rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

        let mut certs_file = BufReader::new(File::open(tls.cert).map_err(
            |error| {
                log::error!("Failed to open certificate file: {}", error.to_string());
                std::process::exit(1);
            }
        ).unwrap());
        let mut key_file = BufReader::new(File::open(tls.key).map_err(
            |error| {
                log::error!("Failed to open key file: {}", error.to_string());
                std::process::exit(1);
            }
        ).unwrap());

        let tls_certs = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .next()
            .unwrap()
            .unwrap();

        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .unwrap();

        server = server.listen_rustls_0_23(listener, tls_config).unwrap();
    } else {
        server = server.listen(listener).unwrap();
    }

    server.addrs_with_scheme().iter().for_each(|addr| {
        let (socket_addr, str_ref) = addr;
        log::info!("🚀 Http Server started at {}://{:?}", str_ref, socket_addr);
    });
    server.workers(workers).run().await.map(|_| {
        log::info!("Http Server stopped!")
    })?;
    Ok(())
}
