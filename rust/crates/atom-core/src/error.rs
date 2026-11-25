use std::{io, path::PathBuf};

use serde::Serialize;
use thiserror::Error;

use crate::{config::ConfigError, storage::StorageError};

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Unable to locate platform application directories.")]
    DirectoryDiscovery,
    #[error("Unable to prepare the {kind} application directory.")]
    DirectoryCreation {
        kind: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    Configuration(#[from] ConfigError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IpcErrorCode {
    Configuration,
    ApplicationDirectories,
    LocalStorage,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpcError {
    pub code: IpcErrorCode,
    pub message: &'static str,
}

impl From<CoreError> for IpcError {
    fn from(error: CoreError) -> Self {
        let (code, message) = match error {
            CoreError::DirectoryDiscovery | CoreError::DirectoryCreation { .. } => (
                IpcErrorCode::ApplicationDirectories,
                "ATOM Studio could not prepare its application directories.",
            ),
            CoreError::Configuration(_) => (
                IpcErrorCode::Configuration,
                "ATOM Studio configuration is invalid or unavailable.",
            ),
            CoreError::Storage(_) => (
                IpcErrorCode::LocalStorage,
                "ATOM Studio could not initialize local storage.",
            ),
        };

        Self { code, message }
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreError, IpcError, IpcErrorCode};
    use crate::test_support::TestDir;
    use std::{io, path::PathBuf};

    #[test]
    fn ipc_error_serialization_is_stable_and_does_not_expose_paths() {
        let private_path = PathBuf::from("C:/private/user/config.toml");
        let error = IpcError::from(CoreError::DirectoryCreation {
            kind: "configuration",
            path: private_path,
            source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
        });
        let response = serde_json::to_value(error).expect("serialize IPC error");

        assert_eq!(response["code"], "application_directories");
        assert_eq!(
            response["message"],
            "ATOM Studio could not prepare its application directories."
        );
        assert!(!response.to_string().contains("private"));
    }

    #[test]
    fn configuration_failures_have_a_sanitized_ipc_category() {
        let temp = TestDir::new();
        let failure = crate::config::AppConfig::load_with_override(
            &temp.path().join("config.toml"),
            Some("not-a-level"),
        )
        .expect_err("invalid values must fail");
        let response = IpcError::from(CoreError::Configuration(failure));

        assert_eq!(response.code, IpcErrorCode::Configuration);
        assert!(!response.message.contains("not-a-level"));
    }
}
