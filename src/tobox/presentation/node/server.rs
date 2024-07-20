use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{App, HttpServer as ActixHttpServer, web};
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;

use crate::application::common::server::{ConnectionConfig, Server};
use crate::domain::models::service::ServiceTextId;
use crate::ioc::IoC;
use crate::presentation;
use crate::presentation::node::interactor_factory::InteractorFactory;


pub struct NodeServer {
    connection_config: Arc<Mutex<ConnectionConfig>>,
    version: &'static str,
    is_intermediate: bool
}

impl NodeServer {
    pub fn new(
        version: &'static str,
        is_intermediate: bool
    ) -> Self {
        Self {
            connection_config: Arc::new(Mutex::new(ConnectionConfig::default())),
            version,
            is_intermediate
        }
    }
}

impl Server for NodeServer {
    fn bind(self, addr: String) -> io::Result<Self> {
        self.connection_config.lock().unwrap().tcp_listener = Some(
            TcpListener::bind(addr)?
        );
        Ok(self)
    }

    fn set_workers(self, workers: usize) -> Self {
        self.connection_config.lock().unwrap().workers = workers;
        self
    }

    fn set_tls(self, key: &str, cert: &str) -> Self {
        self.connection_config.lock().unwrap().tls = Some((key.to_string(), cert.to_string()));
        self
    }
    
    fn run(self) -> io::Result<()> {
        let rt = actix_web::rt::Runtime::new().unwrap();
        rt.block_on(async {
            let ioc = self.ioc.clone();
            let app_config_provider = self.app_config_provider.clone();

            let app_builder = move || {
                let ioc_arc: Arc<dyn InteractorFactory> = ioc.clone();
                let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);

                App::new()
                    .service(web::scope("/node")
                        .configure(presentation::panel::rest::user::router)
                        .configure(presentation::panel::rest::session::router)
                        .configure(presentation::panel::rest::access_log::router)
                        .configure(presentation::panel::rest::role::router)
                        .configure(presentation::panel::rest::stats::router)
                        .configure(presentation::panel::rest::permission::router)
                        .configure(presentation::panel::rest::service::router)
                    )
                    .app_data(web::Data::new(
                        app_config_provider.clone()
                    ))
                    .app_data(ioc_data)
                    .default_service(web::route().to(presentation::panel::exception::not_found))
                    .wrap(Logger::default())
            };

            let available_workers = {
                let connection_config = self.connection_config.lock().unwrap();
                connection_config.workers
            };

            let tcp_listener = {
                let connection_config = self.connection_config.lock().unwrap();
                match connection_config.tcp_listener.as_ref() {
                    Some(tcp_listener) => tcp_listener.try_clone().unwrap(),
                    None => {
                        log::error!("TcpListener is not set");
                        std::process::exit(1);
                    },
                }
            };
            
            let tls = {
                let connection_config = self.connection_config.lock().unwrap();
                connection_config.tls.clone()
            };

            let mut server = ActixHttpServer::new(app_builder);
            if let Some(tls) = tls {
                rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

                let mut key_file = BufReader::new(File::open(tls.0).map_err(
                    |error| {
                        log::error!("Failed to open key file: {}", error.to_string());
                        std::process::exit(1);
                    }
                ).unwrap());

                let mut certs_file = BufReader::new(File::open(tls.1).map_err(
                    |error| {
                        log::error!("Failed to open certificate file: {}", error.to_string());
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

                server = server.listen_rustls_0_23(tcp_listener, tls_config).unwrap();
            } else {
                server = server.listen(tcp_listener).unwrap();
            }

            server.addrs_with_scheme().iter().for_each(|addr| {
                let (socket_addr, str_ref) = addr;
                log::info!("🚀 NodeServer started at {}://{:?}", str_ref, socket_addr);
            });

            server.workers(available_workers).run().await.and_then(|_| {
                log::info!("NodeServer stopped!");
                Ok(())
            }).unwrap();
        });
        Ok(())
    }
}