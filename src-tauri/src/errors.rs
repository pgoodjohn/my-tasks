use crate::configuration::manager::ConfigurationMode;
use std::string::ToString;
use strum_macros::Display;
use strum_macros::EnumString;

pub fn handle_error(error: &dyn std::error::Error) -> String {
    log::error!("{:?}", error);
    let sentry_id = sentry::capture_error(error);
    log::debug!("Error logged to Sentry: {}", sentry_id);
    error.to_string()
}

#[derive(Debug, EnumString, Display)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl TryFrom<ConfigurationMode> for Environment {
    type Error = ();

    fn try_from(mode: ConfigurationMode) -> Result<Self, Self::Error> {
        match mode {
            ConfigurationMode::Development => Ok(Environment::Development),
            ConfigurationMode::Desktop => Ok(Environment::Production),
            _ => Ok(Environment::Testing),
        }
    }
}

pub struct ErrorHandler {
    _sentry: sentry::ClientInitGuard,
}

impl ErrorHandler {
    pub fn initialise(mode: ConfigurationMode) -> Self {
        let environment = Environment::try_from(mode).unwrap();

        let guard = sentry::init((
            dotenv!("SENTRY_DNS"),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(environment.to_string().into()),
                ..Default::default()
            },
        ));

        Self { _sentry: guard }
    }
}
