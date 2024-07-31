use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer as ActixHttpServer};
use actix_web::middleware::Logger;

use crate::application::common::server::{ConnectionConfig, Server};
use crate::presentation;


pub struct PanelServer {
    connection_config: Arc<Mutex<ConnectionConfig>>
}

impl PanelServer {
    pub fn new() -> Self {
        Self {
            connection_config: Arc::new(Mutex::new(ConnectionConfig::default())),
        }
    }
}

impl Server for PanelServer {
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

            let app_builder = move || {
                App::new()
                    .configure(presentation::panel::routes::router)
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
                        log::error!("Failed to open key object: {}", error.to_string());
                        std::process::exit(1);
                    }
                ).unwrap());
                
                let mut certs_file = BufReader::new(File::open(tls.1).map_err(
                    |error| {
                        log::error!("Failed to open certificate object: {}", error.to_string());
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
                log::info!("🚀 PanelServer started at {}://{:?}", str_ref, socket_addr);
            });

            server.workers(available_workers).run().await.and_then(|_| {
                log::info!("PanelServer stopped!");
                Ok(())
            }).unwrap();
        });
        Ok(())
    }
}