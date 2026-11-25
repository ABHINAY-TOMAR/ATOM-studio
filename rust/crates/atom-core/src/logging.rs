use std::{error::Error, path::Path};

use thiserror::Error;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::filter::LevelFilter;

use crate::config::LogLevel;

const RETAINED_LOG_FILES: usize = 7;

#[derive(Debug, Error)]
pub enum LoggingError {
    #[error("Unable to open the application log directory.")]
    File(#[source] tracing_appender::rolling::InitError),
    #[error("Application logging has already been initialized.")]
    Subscriber(#[source] Box<dyn Error + Send + Sync>),
}

impl LoggingError {
    pub fn diagnostic_code(&self) -> &'static str {
        match self {
            Self::File(_) => "log_file_initialization",
            Self::Subscriber(_) => "logging_subscriber_initialization",
        }
    }
}

pub fn initialize(log_directory: &Path, level: LogLevel) -> Result<WorkerGuard, LoggingError> {
    let appender = rolling::Builder::new()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix("atom-studio")
        .filename_suffix("log")
        .max_log_files(RETAINED_LOG_FILES)
        .build(log_directory)
        .map_err(LoggingError::File)?;
    let (writer, guard) = tracing_appender::non_blocking(appender);
    let filter = match level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };

    tracing_subscriber::fmt()
        .with_max_level(filter)
        .with_ansi(false)
        .with_writer(writer)
        .try_init()
        .map_err(LoggingError::Subscriber)?;

    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::{initialize, LogLevel, LoggingError};
    use crate::test_support::TestDir;
    use std::fs;

    #[test]
    fn logging_errors_expose_a_safe_diagnostic_category() {
        let error = LoggingError::Subscriber(Box::new(std::io::Error::other("private detail")));

        assert_eq!(error.diagnostic_code(), "logging_subscriber_initialization");
        assert_eq!(
            error.to_string(),
            "Application logging has already been initialized."
        );
        assert!(!error.to_string().contains("private detail"));
    }

    #[test]
    fn writes_diagnostics_only_to_the_supplied_log_directory() {
        let temp = TestDir::new();
        let log_directory = temp.path().join("logs");
        fs::create_dir_all(&log_directory).expect("create log directory");

        let guard = initialize(&log_directory, LogLevel::Info).expect("initialize file logging");
        tracing::info!(event = "test_startup", "application started");
        drop(guard);

        let entries = fs::read_dir(&log_directory)
            .expect("read log directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect log files");
        assert_eq!(entries.len(), 1);
        let log = fs::read_to_string(entries[0].path()).expect("read application log");
        assert!(log.contains("application started"));
        assert!(!log.contains("api_key"));
    }
}
