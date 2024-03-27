use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DistributeMode {
    Single,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub mode: DistributeMode,
    pub storages: Vec<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Result<Config, String> {

        if !std::path::Path::new(config_path).exists() {
            let default_config = Config {
                host: "127.0.0.1".to_string(),
                port: 8080,
                mode: DistributeMode::Single,
                storages: vec![],
            };
            let file = OpenOptions::new().write(true).create_new(true).open(config_path);
            match file {
                Ok(mut file) => {
                    if let Err(error) = writeln!(file, "{}", serde_yaml::to_string(&default_config).unwrap()) {
                        return Err(format!("Failed to write to config.yaml -> {}", error));
                    }
                },
                Err(error) => return Err(format!("Failed to create config.yaml -> {}", error)),
            };
        }


        let mut file = match File::open(config_path) {
            Ok(file) => file,
            Err(error) => return Err(format!("Failed to open config.yaml -> {}", error)),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to read config.yaml -> {}", error)),
        };

        let config: Config = match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(error) => {
                return Err(format!("Failed to parse config.yaml -> {}", error));
            },
        };
        Ok(config)
    }
}