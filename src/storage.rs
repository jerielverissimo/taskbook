use super::config::*;

use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Storage {
    _storage_dir: PathBuf,
    _archive_dir: PathBuf,
    _temp_dir: PathBuf,
    _archive_file: PathBuf,
    _main_storage_file: PathBuf,
    _config: Config,
}

impl Storage {
    pub fn new() -> Self {
        let mut storage = Storage {
            _storage_dir: PathBuf::from(""),
            _archive_dir: PathBuf::from(""),
            _temp_dir: PathBuf::from(""),
            _archive_file: PathBuf::from(""),
            _main_storage_file: PathBuf::from(""),
            _config: Config::new(),
        };

        storage._storage_dir = storage._main_dir().join("storage");
        storage._archive_dir = storage._main_dir().join("archive");
        storage._temp_dir = storage._main_dir().join(".tmp");

        storage
    }

    pub fn get_archive(&self) {
        unimplemented!()
    }

    fn _main_dir(&self) -> PathBuf {
        let taskbook_derictory = self._config.get().configuration.default.taskbook_derictory;
        let default_app_directory = env::home_dir().unwrap().join(".taskbook");

        if taskbook_derictory.exists() {
            return default_app_directory;
        }

        taskbook_derictory
    }
}
