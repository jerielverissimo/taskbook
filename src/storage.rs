use super::config::*;
use super::item::Item;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use serde_traitobject as s;
use uuid::Uuid;

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
        storage._archive_file = storage._archive_dir.join("archive.json");
        storage._main_storage_file = storage._storage_dir.join("storage.json");

        storage._ensure_directories();
        storage
    }

    pub fn get(&self) -> Vec<s::Box<dyn Item>> {
        let mut items: Vec<s::Box<dyn Item>> = Vec::new();
        if self._main_storage_file.exists() {
            let mut contents = String::new();
            let mut file = fs::File::open(&self._main_storage_file).unwrap();
            file.read_to_string(&mut contents).unwrap();
            items = serde_json::from_str(&contents).unwrap();
        }
        items
    }

    pub fn get_archive(&self) -> Vec<s::Box<dyn Item>> {
        let mut archive: Vec<s::Box<dyn Item>> = Vec::new();
        if self._archive_file.exists() {
            let mut contents = String::new();
            let mut file = fs::File::open(&self._archive_file).unwrap();
            file.read_to_string(&mut contents).unwrap();
            archive = serde_json::from_str(&contents).unwrap();
        }
        archive
    }

    pub fn set(&self, data: String) {
        let tmp_storage_file = self._get_temp_file(&self._main_storage_file);

        fs::write(&tmp_storage_file, data).unwrap();
        fs::rename(tmp_storage_file, &self._main_storage_file).unwrap();
    }

    pub fn set_archive(&self, archive: String) {
        let tmp_archive_file = self._get_temp_file(&self._archive_file);

        fs::write(&tmp_archive_file, archive).unwrap();
        fs::rename(tmp_archive_file, &self._archive_file).unwrap();
    }

    fn _main_dir(&self) -> PathBuf {
        let taskbook_derictory = self._config.get().configuration.default.taskbook_derictory;
        let default_app_directory = env::home_dir().unwrap().join(".taskbook");

        if taskbook_derictory.exists() {
            return default_app_directory;
        }

        taskbook_derictory
    }

    fn _ensure_storage_dir(&self) {
        if !self._storage_dir.exists() {
            fs::create_dir(&self._storage_dir).unwrap();
        }
    }

    fn _ensure_temp_dir(&self) {
        if !self._temp_dir.exists() {
            fs::create_dir(&self._temp_dir).unwrap();
        }
    }

    fn _ensure_archive_dir(&self) {
        if !self._archive_dir.exists() {
            fs::create_dir(&self._archive_dir).unwrap();
        }
    }

    fn _clean_temp_dir(&self) {
        let temp_files = fs::read_dir(&self._temp_dir)
            .unwrap()
            .map(|x| x.unwrap().path());

        temp_files.for_each(|tmp| fs::remove_file(tmp).unwrap());
    }

    fn _ensure_directories(&self) {
        self._ensure_storage_dir();
        self._ensure_temp_dir();
        self._ensure_archive_dir();
        self._clean_temp_dir();
    }

    fn _get_random_hex_string(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn _get_temp_file(&self, file_path: &PathBuf) -> PathBuf {
        let ramdom_string = self._get_random_hex_string();
        let tmp_file_name = file_path
            .strip_prefix(".")
            .unwrap()
            .join(format!(".TEMP-{}", ramdom_string));
        self._temp_dir.join(tmp_file_name)
    }
}
