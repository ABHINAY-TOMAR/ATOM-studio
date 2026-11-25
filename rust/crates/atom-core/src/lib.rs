pub mod config;
pub mod error;
pub mod logging;
pub mod paths;
pub mod storage;

use config::AppConfig;
use error::CoreError;
use paths::AppPaths;
use storage::LocalStorage;

#[derive(Debug)]
pub struct ApplicationCore {
    config: AppConfig,
    paths: AppPaths,
    storage: LocalStorage,
}

impl ApplicationCore {
    pub fn initialize() -> Result<Self, CoreError> {
        let paths = AppPaths::discover()?;
        paths.ensure()?;
        let config = AppConfig::load(&paths.config_file())?;

        Self::from_initialized_paths(paths, config)
    }

    pub fn initialize_with_paths(
        paths: AppPaths,
        environment_log_level: Option<&str>,
    ) -> Result<Self, CoreError> {
        paths.ensure()?;
        let config = AppConfig::load_with_override(&paths.config_file(), environment_log_level)?;

        Self::from_initialized_paths(paths, config)
    }

    fn from_initialized_paths(paths: AppPaths, config: AppConfig) -> Result<Self, CoreError> {
        let storage = LocalStorage::initialize(paths.data_dir().join("storage"))?;

        Ok(Self {
            config,
            paths,
            storage,
        })
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn paths(&self) -> &AppPaths {
        &self.paths
    }

    pub fn storage(&self) -> &LocalStorage {
        &self.storage
    }
}

pub fn ipc_smoke_response(name: &str) -> String {
    format!("Hello, {name} — Rust IPC is working.")
}

#[cfg(test)]
mod tests {
    use super::{ipc_smoke_response, ApplicationCore};
    use crate::{config::LogLevel, error::CoreError, paths::AppPaths, test_support::TestDir};
    use std::fs;

    #[test]
    fn core_initializes_from_isolated_paths_and_keeps_ipc_contract() {
        let temp = TestDir::new();
        let paths = AppPaths::from_root(temp.path().join("app"));
        fs::create_dir_all(paths.config_dir()).expect("create config directory");
        fs::write(paths.config_file(), "log_level = 'debug'\n").expect("write config");

        let core = ApplicationCore::initialize_with_paths(paths, None)
            .expect("core should initialize with valid local settings");

        assert_eq!(core.config().log_level, LogLevel::Debug);
        assert!(core.storage().root().is_dir());
        assert_eq!(
            ipc_smoke_response("ATOM Studio"),
            "Hello, ATOM Studio — Rust IPC is working."
        );
    }

    #[test]
    fn invalid_configuration_prevents_core_initialization() {
        let temp = TestDir::new();
        let paths = AppPaths::from_root(temp.path().join("app"));
        fs::create_dir_all(paths.config_dir()).expect("create config directory");
        fs::write(paths.config_file(), "log_level = 'verbose'\n").expect("write config");

        let error = ApplicationCore::initialize_with_paths(paths, None)
            .expect_err("invalid configuration must stop initialization");

        assert!(matches!(error, CoreError::Configuration(_)));
    }

    #[test]
    fn existing_ipc_helper_behavior_remains_unchanged() {
        assert_eq!(
            ipc_smoke_response("ATOM Studio"),
            "Hello, ATOM Studio — Rust IPC is working."
        );
    }
}

#[cfg(test)]
mod test_support {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    pub struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        pub fn new() -> Self {
            let base = std::env::temp_dir();
            loop {
                let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
                let path = base.join(format!("atom-core-test-{}-{id}", std::process::id()));
                match fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("failed to create test directory: {error}"),
                }
            }
        }

        pub fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
