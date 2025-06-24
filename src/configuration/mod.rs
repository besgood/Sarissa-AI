
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct SarissaConfig {
    pub enable_ai: bool,
    pub default_ports: String,
    pub output_path: String,
}

impl SarissaConfig {
    pub fn load_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&data).expect("Failed to parse config file")
    }
}
