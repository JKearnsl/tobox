use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use std::sync::RwLock;
use tokio::sync::RwLock as AsyncRwLock;
use tokio::fs::OpenOptions as AsyncOpenOptions;
use tokio::io::AsyncWriteExt;
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
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PanelConfig {
    pub host: String,
    pub port: u16,
    pub tls: Option<Tls>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialsConfig {
    pub username: String,
    pub password: Option<String>,
    pub hashed_password: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub tls: Option<Tls>,
    pub disk: DiskConfig,
    pub session_ttl: u32,
    pub cluster: Option<ClusterConfig>,
    pub credentials: Option<CredentialsConfig>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub log_level: Option<String>,
    pub panel: Option<PanelConfig>,
    pub node: Option<NodeConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
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
                session_ttl: 3600,
                disk: DiskConfig {
                    path: "path/to/mount/point".to_string()
                },
                cluster: None,
                credentials: None
            })
        }
    }
}

pub struct ConfigManager {
    config: RwLock<Config>,
    path: String,
}

impl ConfigManager {
    pub fn from_file<T: Into<String>>(config_path: T) -> Self {
        let config_path = &config_path.into();
        
        if !std::path::Path::new(config_path).exists() {
            match OpenOptions::new().write(true).create_new(true).open(config_path) {
                Ok(mut file) => {
                    if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&Config::default()).unwrap()) {
                        log::error!("Failed to write to {}: {}", config_path, error);
                        std::process::exit(1);
                    }
                },
                Err(error) => {
                    log::error!("Failed to create {}: {}", config_path, error);
                    std::process::exit(1);
                },
            };
        }

        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(error) => { 
                log::error!("Failed to open {}: {}", config_path, error);
                std::process::exit(1);
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(error) => { 
                log::error!("Failed to read from {}: {}", config_path, error);
                std::process::exit(1);
            }
        };

        let config = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(error) => {
                log::error!("Failed to parse {}: {}", config_path, error);
                std::process::exit(1);
            },
        };
        
        Self {
            config: RwLock::new(config),
            path: config_path.to_string(),
        }
    }
    
    pub fn get(&self) -> Config {
        self.config.read().unwrap().clone()
    }
    
    pub fn set(&self, config: Config) {
        self.config.write().unwrap().clone_from(&config);
    }
    
    pub fn save(&self) -> Result<(), String> {
        let config = self.config.read().unwrap().clone();
        let mut file = match OpenOptions::new().write(true).truncate(true).open(&self.path) {
            Ok(file) => file,
            Err(error) => return Err(format!("Failed to open {}: {}", self.path, error)),
        };
        if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&config).unwrap()) {
            return Err(format!("Failed to write to {}: {}", self.path, error));
        }
        Ok(())
    }
}

pub struct AsyncConfigManager {
    config: AsyncRwLock<Config>,
    path: String,
}

impl AsyncConfigManager {
    pub async fn from_file<T: Into<String>>(config_path: T) -> Self {
        let config_path = &config_path.into();

        if !std::path::Path::new(config_path).exists() {
            match OpenOptions::new().write(true).create_new(true).open(config_path) {
                Ok(mut file) => {
                    if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&Config::default()).unwrap()) {
                        log::error!("Failed to write to {}: {}", config_path, error);
                        std::process::exit(1);
                    }
                },
                Err(error) => {
                    log::error!("Failed to create {}: {}", config_path, error);
                    std::process::exit(1);
                },
            };
        }

        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(error) => {
                log::error!("Failed to open {}: {}", config_path, error);
                std::process::exit(1);
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(error) => {
                log::error!("Failed to read from {}: {}", config_path, error);
                std::process::exit(1);
            }
        };

        let config = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(error) => {
                log::error!("Failed to parse {}: {}", config_path, error);
                std::process::exit(1);
            },
        };

        Self {
            config: AsyncRwLock::new(config),
            path: config_path.to_string(),
        }
    }

    pub async fn get(&self) -> Config {
        self.config.read().await.clone()
    }

    pub async fn set(&self, config: Config) {
        self.config.write().await.clone_from(&config);
    }

    pub async fn save(&self) -> Result<(), String> {
        let config = self.config.read().await.clone();
        let mut file = match AsyncOpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await 
        {
            Ok(file) => file,
            Err(error) => return Err(format!("Failed to open {}: {}", self.path, error)),
        };
        if let Err(error) = file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes()).await {
            return Err(format!("Failed to write to {}: {}", self.path, error));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new("test_config.yaml").unwrap();
        assert_eq!(config.panel.is_some(), true);
        assert_eq!(config.node.is_some(), true);
    }

    #[test]
    fn test_config_manager() {
        let config_manager = ConfigManager::new("test_config.yaml");
        let config = config_manager.get();
        assert_eq!(config.panel.is_some(), true);
        assert_eq!(config.node.is_some(), true);
    }
}
