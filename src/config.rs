use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterMode {
    Combined,
    Ec(u8)
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterConfig {
    pub mode: ClusterMode,
    pub nodes: Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiskConfig {
    pub mode: ClusterMode,
    pub volumes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PanelConfig {
    pub host: String,
    pub port: u16,
    pub tls: Option<Tls>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub is_intermediate: bool,
    pub tls: Option<Tls>,
    pub disk: DiskConfig,
    pub cluster: Option<ClusterConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    config_path: String,
    pub log_level: Option<String>,
    pub panel: Option<PanelConfig>,
    pub node: Option<NodeConfig>,
}

impl Config {
    pub fn new(config_path: &str) -> Result<Self, String> {
        if !std::path::Path::new(config_path).exists() {
            let default_config = Self {
                config_path: config_path.to_string(),
                log_level: Some("info".to_string()),
                panel: Some(PanelConfig {
                    host: "127.0.0.1".to_string(),
                    port: 8080,
                    tls: None
                }),
                node: Some(NodeConfig {
                    host: "127.0.0.1".to_string(),
                    port: 3030,
                    tls: None,
                    workers: None,
                    is_intermediate: false,
                    disk: DiskConfig {
                        mode: ClusterMode::Combined,
                        volumes: vec![]
                    },
                    cluster: None
                })
            };

            match OpenOptions::new().write(true).create_new(true).open(config_path) {
                Ok(mut file) => {
                    if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&default_config).unwrap()) {
                        return Err(format!("Failed to write to {} -> {}", config_path, error));
                    }
                },
                Err(error) => return Err(format!("Failed to create {} -> {}", config_path, error)),
            };
        }
        
        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(error) => return Err(format!("Failed to open {} -> {}", config_path, error)),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to read {} -> {}", config_path, error)),
        };

        let config = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(error) => {
                return Err(format!("Failed to parse {} -> {}", config_path, error));
            },
        };
        Ok(config)
    }
    
    pub fn save(&self, config: Config) -> Result<(), String> {
        match OpenOptions::new().write(true).truncate(true).open(&config.config_path) {
            Ok(mut file) => {
                if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&config).unwrap()) {
                    return Err(format!("Failed to write to {} -> {}", config.config_path, error));
                }
            },
            Err(error) => return Err(format!("Failed to open {} -> {}", config.config_path, error)),
        };
        Ok(())
    }
}