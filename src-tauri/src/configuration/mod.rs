use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod commands;
pub mod manager;
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
