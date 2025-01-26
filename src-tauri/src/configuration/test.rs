#[cfg(test)]
mod configuration_manager_tests {
    use crate::configuration::manager::{ConfigurationManager, ConfigurationMode};

    fn test_cleanup() {
        let _ = std::fs::remove_file(".test-config.toml");
    }

    #[test]
    fn it_intializes_a_configuration_when_there_is_no_file() {
        test_cleanup();
        let configuration_manager = ConfigurationManager::init(ConfigurationMode::Test);

        assert_eq!(
            configuration_manager.configuration.version,
            env!("CARGO_PKG_VERSION")
        );
        assert!(configuration_manager.configuration.development_mode);
        assert_eq!(
            configuration_manager
                .configuration
                .config_path
                .to_str()
                .unwrap(),
            ".test-config.toml"
        );
        assert_eq!(
            configuration_manager
                .configuration
                .db_path
                .to_str()
                .unwrap(),
            "test-file.db"
        );
        test_cleanup();
    }
}
