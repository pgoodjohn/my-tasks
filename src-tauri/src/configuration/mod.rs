use crate::ollama::OllamaConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod manager;
pub mod tauri;
mod test;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub version: String,
    #[serde(rename = "developmentMode")]
    pub development_mode: bool,
    #[serde(rename = "configurationPath")]
    pub config_path: PathBuf,
    #[serde(rename = "dbPath")]
    pub db_path: PathBuf,
    #[serde(default)]
    pub ollama: OllamaConfig,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            development_mode: true,
            config_path: ".config.toml".into(),
            db_path: "file.db".into(),
            ollama: OllamaConfig::default(),
        }
    }
}
