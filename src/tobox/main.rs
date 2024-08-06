use std::str::FromStr;
use std::thread;

use crate::application::common::server::Server;
use node::NodeServer;
use panel::PanelServer;

mod domain;
mod config;
mod presentation;
mod application;
mod adapters;
mod node;
mod panel;
mod ioc;

fn main() -> std::io::Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const CONFIG_PATH: &str = "config.yaml";
    
    let config = config::ConfigManager::from_file(CONFIG_PATH);
    if let Some(log_level) = &config.get().log_level {
        env_logger::builder()
            .filter_level(log::LevelFilter::from_str(log_level).unwrap())
            .init();
    }
    
    thread::scope(|scope| {
        if let Some(panel_config) = &config.get().panel {
            let mut server = PanelServer::new();
            if let Some(tls) = &panel_config.tls {
                server = server.set_tls(&tls.key, &tls.cert);
            }
            server = match server.bind(format!("{}:{}", panel_config.host, panel_config.port)) {
                Ok(server) => server,
                Err(error) => {
                    log::error!(
                        "[Panel] Failed to bind to {}:{}: {}", 
                        panel_config.host, 
                        panel_config.port,
                        error
                    );
                    std::process::exit(1);
                }
            };
            server = server.set_workers(1);

            scope.spawn(move || {
                server.run().unwrap();
            });
        }

        if let Some(node_config) = &config.get().node {
            let mut server = NodeServer::new(VERSION, CONFIG_PATH);
            if let Some(tls) = &node_config.tls {
                server = server.set_tls(&tls.key, &tls.cert);
            }
            server = match server.bind(format!("{}:{}", node_config.host, node_config.port)) {
                Ok(server) => server,
                Err(error) => {
                    log::error!(
                        "[Node] Failed to bind to {}:{}: {}", 
                        node_config.host, 
                        node_config.port,
                        error
                    );
                    std::process::exit(1);
                }
            };
            server = server.set_workers(node_config.workers.unwrap_or_else(|| {
                match thread::available_parallelism() {
                    Ok(parallelism) => usize::from(parallelism),
                    Err(_) => 1,
                }
            }));

            scope.spawn(move || {
                server.run().unwrap();
            });
        }
    });
    
    Ok(())
}
