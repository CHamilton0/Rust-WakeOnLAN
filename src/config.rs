use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub mac: String,
    pub ip: String,
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".magic_packet_config.toml");
    path
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if let Ok(data) = fs::read_to_string(&path) {
        toml::from_str(&data).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

pub fn save_config(cfg: &AppConfig) {
    let path = config_path();
    if let Ok(toml) = toml::to_string(cfg) {
        let _ = fs::write(path, toml);
    }
}
