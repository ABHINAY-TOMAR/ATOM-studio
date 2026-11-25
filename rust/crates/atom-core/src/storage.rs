use std::{fs, io, path::PathBuf};

use thiserror::Error;

#[derive(Debug)]
pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn initialize(root: PathBuf) -> Result<Self, StorageError> {
        fs::create_dir_all(&root).map_err(|source| StorageError::Initialize {
            path: root.clone(),
            source,
        })?;

        Ok(Self { root })
    }

    pub fn root(&self) -> &std::path::Path {
        &self.root
    }
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Unable to initialize the application storage directory.")]
    Initialize {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::{LocalStorage, StorageError};
    use crate::test_support::TestDir;
    use std::fs;

    #[test]
    fn initializes_storage_at_an_application_owned_path() {
        let temp = TestDir::new();
        let root = temp.path().join("data").join("storage");

        let storage = LocalStorage::initialize(root.clone()).expect("initialize local storage");

        assert!(root.is_dir());
        assert_eq!(storage.root(), root);
    }

    #[test]
    fn initialization_fails_if_the_target_is_a_file() {
        let temp = TestDir::new();
        let root = temp.path().join("storage");
        fs::write(&root, "user data").expect("create blocking file");

        let error = LocalStorage::initialize(root).expect_err("file cannot be used as a directory");

        assert!(matches!(error, StorageError::Initialize { .. }));
        assert_eq!(
            fs::read_to_string(temp.path().join("storage")).expect("file remains"),
            "user data"
        );
    }
}
