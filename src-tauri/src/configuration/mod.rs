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
}
