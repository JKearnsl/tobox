use std::str::FromStr;
use std::thread;

use crate::application::common::server::Server;
use crate::presentation::node::server::NodeServer;
use crate::presentation::panel::server::PanelServer;

mod domain;
mod config;
mod presentation;
mod application;
// mod adapters;


fn main() -> std::io::Result<()> {
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
    
    thread::scope(|s| {
        if let Some(panel_config) = &config.panel {
            let mut server = PanelServer::new();
            if let Some(tls) = &panel_config.tls {
                server = server.set_tls(&tls.key, &tls.cert);
            }
            server = server.bind(format!("{}:{}", panel_config.host, panel_config.port)).map_err(
                |error| {
                    log::error!("Failed to bind to {}:{}", panel_config.host, panel_config.port);
                    error
                }
            ).unwrap();
            server = server.set_workers(1);

            s.spawn(move || {
                server.run().unwrap();
            });
        }

        if let Some(node_config) = &config.node {
            let mut server = NodeServer::new(
                env!("CARGO_PKG_VERSION"),
                node_config.is_intermediate
            );
            if let Some(tls) = &node_config.tls {
                server = server.set_tls(&tls.key, &tls.cert);
            }
            server = server.bind(format!("{}:{}", node_config.host, node_config.port)).map_err(
                |error| {
                    log::error!("Failed to bind to {}:{}", node_config.host, node_config.port);
                    error
                }
            ).unwrap();
            server = server.set_workers(node_config.workers.unwrap_or_else(|| {
                match thread::available_parallelism() {
                    Ok(parallelism) => usize::from(parallelism),
                    Err(_) => 1,
                }
            }));

            s.spawn(move || {
                server.run().unwrap();
            });
        }
    });
    
    Ok(())
}
