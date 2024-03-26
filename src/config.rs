use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DistributeMode {
    Single,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub mode: DistributeMode,
    pub storages: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let mut file = match File::open("config.yaml") {
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