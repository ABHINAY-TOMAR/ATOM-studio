use std::{env, fs, io, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

impl FromStr for LogLevel {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(ConfigError::InvalidLogLevel),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct AppConfig {
    pub log_level: LogLevel,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let environment_override = match env::var("ATOM_LOG_LEVEL") {
            Ok(value) => Some(value),
            Err(env::VarError::NotPresent) => None,
            Err(env::VarError::NotUnicode(_)) => return Err(ConfigError::InvalidEnvironmentValue),
        };

        Self::load_with_override(path, environment_override.as_deref())
    }

    pub fn load_with_override(
        path: &Path,
        environment_override: Option<&str>,
    ) -> Result<Self, ConfigError> {
        let mut config = match fs::read_to_string(path) {
            Ok(contents) => toml::from_str(&contents).map_err(ConfigError::Parse)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => Self::default(),
            Err(source) => return Err(ConfigError::Read { source }),
        };

        if let Some(value) = environment_override {
            config.log_level = value.parse()?;
        }

        Ok(config)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Unable to read the application configuration file.")]
    Read {
        #[source]
        source: io::Error,
    },
    #[error("The application configuration file is invalid.")]
    Parse(#[source] toml::de::Error),
    #[error("The configured log level is invalid.")]
    InvalidLogLevel,
    #[error("ATOM_LOG_LEVEL must contain a valid Unicode log level.")]
    InvalidEnvironmentValue,
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, ConfigError, LogLevel};
    use crate::test_support::TestDir;

    #[test]
    fn missing_configuration_uses_defaults() {
        let temp = TestDir::new();
        let config = AppConfig::load_with_override(&temp.path().join("missing.toml"), None)
            .expect("a missing config file should use defaults");

        assert_eq!(config, AppConfig::default());
        assert_eq!(config.log_level, LogLevel::Info);
    }

    #[test]
    fn environment_override_takes_precedence_over_file_value() {
        let temp = TestDir::new();
        let path = temp.path().join("config.toml");
        std::fs::write(&path, "log_level = 'warn'\n").expect("write test config");

        let config = AppConfig::load_with_override(&path, Some("debug"))
            .expect("valid environment override should load");

        assert_eq!(config.log_level, LogLevel::Debug);
    }

    #[test]
    fn invalid_environment_override_is_rejected_without_echoing_its_value() {
        let temp = TestDir::new();
        let error = AppConfig::load_with_override(
            &temp.path().join("missing.toml"),
            Some("secret-looking-invalid-value"),
        )
        .expect_err("invalid log levels must not be accepted");

        assert!(matches!(error, ConfigError::InvalidLogLevel));
        assert_eq!(error.to_string(), "The configured log level is invalid.");
    }

    #[test]
    fn unknown_configuration_keys_are_rejected() {
        let temp = TestDir::new();
        let path = temp.path().join("config.toml");
        std::fs::write(&path, "log_level = 'info'\nunknown = true\n").expect("write test config");

        assert!(matches!(
            AppConfig::load_with_override(&path, None),
            Err(ConfigError::Parse(_))
        ));
    }
}
