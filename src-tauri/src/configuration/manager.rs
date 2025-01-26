use uuid::Uuid;

use crate::configuration::Configuration;
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum ConfigurationMode {
    Development,
    Desktop,
    Test,
    _IOs,
}

#[derive(Debug, Clone)]
pub struct ConfigurationManager {
    pub storage_manager: ConfigurationStorageManager,
    pub configuration: Configuration,
}

#[derive(Debug, Clone)]
pub struct ConfigurationStorageManager {
    pub configuration_path: PathBuf,
    pub db_path: PathBuf,
}

impl ConfigurationManager {
    pub fn _for_configuration(configuration: Configuration) -> Self {
        let storage_manager = ConfigurationStorageManager {
            configuration_path: configuration.config_path.clone(),
            db_path: configuration.db_path.clone(),
        };

        Self {
            storage_manager,
            configuration,
        }
    }

    pub fn init(mode: ConfigurationMode) -> Self {
        let storage_manager = ConfigurationStorageManager::init(mode);
        let configuration_string = storage_manager.read_from_file().unwrap();

        match toml::from_str::<Configuration>(&configuration_string) {
            Ok(configuration) => Self {
                storage_manager,
                configuration,
            },
            Err(_) => {
                let configuration = Configuration {
                    version: String::from(env!("CARGO_PKG_VERSION")),
                    development_mode: cfg!(debug_assertions),
                    config_path: storage_manager.configuration_path.clone(),
                    db_path: storage_manager.db_path.clone(),
                };

                let _ = storage_manager.write_to_file(
                    toml::to_string(&configuration).expect("Could not serialize config"),
                );
                Self {
                    storage_manager,
                    configuration,
                }
            }
        }
    }

    pub fn load(mode: ConfigurationMode) -> Result<Self, ()> {
        let storage_manager = ConfigurationStorageManager::init(mode);

        match storage_manager.validate_config_path_exists() {
            Err(_) => return Err(()),
            Ok(_) => {
                let configuration_string = storage_manager.read_from_file().unwrap();
                match toml::from_str::<Configuration>(&configuration_string) {
                    Ok(configuration) => Ok(Self {
                        storage_manager,
                        configuration,
                    }),
                    Err(_) => Err(()),
                }
            }
        }
    }

    fn _save_configuration(&self) -> Result<(), ()> {
        let _ = self.storage_manager.write_to_file(
            toml::to_string(&self.configuration).expect("Could not serialize config"),
        );

        Ok(())
    }
}

impl ConfigurationStorageManager {
    pub fn init(mode: ConfigurationMode) -> Self {
        ConfigurationStorageManager {
            configuration_path: ConfigurationStorageManager::configuration_path(mode),
            db_path: ConfigurationStorageManager::db_path(mode),
        }
    }

    fn db_path(mode: ConfigurationMode) -> PathBuf {
        let mut db_path = PathBuf::new();
        match mode {
            ConfigurationMode::Development => db_path.push("file.db"),
            ConfigurationMode::Test => db_path.push("test-file.db"),
            ConfigurationMode::Desktop => {
                db_path.push(dirs::home_dir().expect("Could not load home dir"));
                db_path.push(".config/.my-tasks/db.sqlite");
            }
            _ => todo!("Implement"),
        }

        db_path
    }

    fn configuration_path(mode: ConfigurationMode) -> PathBuf {
        let mut config_path = PathBuf::new();
        match mode {
            ConfigurationMode::Development => config_path.push(".config.toml"),
            ConfigurationMode::Test => config_path.push(".test-config.toml"),
            ConfigurationMode::Desktop => {
                config_path.push(dirs::home_dir().expect("Could not load home dir"));
                config_path.push(".config/.my-tasks/config.toml");
            }
            _ => {
                todo!("Implement")
            }
        }

        config_path
    }

    fn read_from_file(&self) -> Result<String, String> {
        self.ensure_config_file_exists().unwrap();
        std::fs::read_to_string(self.configuration_path.clone()).map_err(|e| e.to_string())
    }

    fn write_to_file(&self, configuration_string: String) -> Result<(), String> {
        let _ = std::fs::write(self.configuration_path.clone(), configuration_string)
            .map_err(|e| e.to_string());

        Ok(())
    }

    fn validate_config_path_exists(&self) -> Result<(), ()> {
        let config_path = self.configuration_path.clone();
        if !config_path.exists() {
            log::error!("Config path does not exist: {:?}", config_path);
            return Err(());
        }
        Ok(())
    }

    fn ensure_config_file_exists(&self) -> Result<(), ()> {
        let config_path = self.configuration_path.clone();

        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                log::info!("Creating configuration directory for {:?}", &config_path);
                std::fs::create_dir_all(parent).expect("Could not create configuration directory");
                println!("Directory created: {:?}", parent);
            }
        }

        if !config_path.exists() {
            log::info!("Creating configuration file {:?}", &config_path);
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&config_path)
                .expect("Could not create config file with write permissions");
        }

        Ok(())
    }
}
