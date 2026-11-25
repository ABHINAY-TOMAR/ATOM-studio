use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::error::CoreError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppPaths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    logs_dir: PathBuf,
    cache_dir: PathBuf,
    temporary_dir: PathBuf,
}

impl AppPaths {
    pub fn discover() -> Result<Self, CoreError> {
        let project = ProjectDirs::from("com", "atomstudio", "ATOM Studio")
            .ok_or(CoreError::DirectoryDiscovery)?;

        Ok(Self {
            config_dir: project.config_dir().to_path_buf(),
            data_dir: project.data_local_dir().to_path_buf(),
            logs_dir: project.data_local_dir().join("logs"),
            cache_dir: project.cache_dir().to_path_buf(),
            temporary_dir: project.cache_dir().join("tmp"),
        })
    }

    pub fn from_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let config_dir = root.join("config");
        let data_dir = root.join("data");
        let cache_dir = root.join("cache");

        Self {
            logs_dir: data_dir.join("logs"),
            temporary_dir: cache_dir.join("tmp"),
            config_dir,
            data_dir,
            cache_dir,
        }
    }

    pub fn ensure(&self) -> Result<(), CoreError> {
        let directories = [
            ("configuration", &self.config_dir),
            ("data", &self.data_dir),
            ("logs", &self.logs_dir),
            ("cache", &self.cache_dir),
            ("temporary", &self.temporary_dir),
        ];

        for (kind, path) in directories {
            fs::create_dir_all(path).map_err(|source| CoreError::DirectoryCreation {
                kind,
                path: path.clone(),
                source,
            })?;
        }

        Ok(())
    }

    pub fn config_dir(&self) -> &std::path::Path {
        &self.config_dir
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn data_dir(&self) -> &std::path::Path {
        &self.data_dir
    }

    pub fn logs_dir(&self) -> &std::path::Path {
        &self.logs_dir
    }

    pub fn cache_dir(&self) -> &std::path::Path {
        &self.cache_dir
    }

    pub fn temporary_dir(&self) -> &std::path::Path {
        &self.temporary_dir
    }
}

#[cfg(test)]
mod tests {
    use super::AppPaths;
    use crate::{error::CoreError, test_support::TestDir};
    use std::fs;

    #[test]
    fn ensures_all_application_directories_without_removing_existing_files() {
        let temp = TestDir::new();
        let paths = AppPaths::from_root(temp.path().join("application"));
        let marker = paths.config_dir().join("user-marker");
        fs::create_dir_all(paths.config_dir()).expect("create config directory");
        fs::write(&marker, "preserve").expect("create marker");

        paths.ensure().expect("create app directories");

        for directory in [
            paths.config_dir(),
            paths.data_dir(),
            paths.logs_dir(),
            paths.cache_dir(),
            paths.temporary_dir(),
        ] {
            assert!(
                directory.is_dir(),
                "expected {} to exist",
                directory.display()
            );
        }
        assert_eq!(fs::read_to_string(marker).expect("read marker"), "preserve");
    }

    #[test]
    fn reports_directory_creation_failure_without_deleting_the_blocking_file() {
        let temp = TestDir::new();
        let root = temp.path().join("application");
        fs::create_dir_all(&root).expect("create test root");
        let config_path = root.join("config");
        fs::write(&config_path, "not a directory").expect("create blocking file");
        let paths = AppPaths::from_root(root);

        let error = paths
            .ensure()
            .expect_err("file must block directory creation");

        assert!(matches!(
            error,
            CoreError::DirectoryCreation {
                kind: "configuration",
                ..
            }
        ));
        assert_eq!(
            fs::read_to_string(config_path).expect("blocking file remains"),
            "not a directory"
        );
    }
}
