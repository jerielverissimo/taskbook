use std::env;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Configuration;

#[derive(Clone)]
pub struct Config {
    _config_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let _config_file = PathBuf::from(env::home_dir().expect("Failed to get home dir :("));
        _config_file.join(".taskbook/taskbook.json");

        let config = Config { _config_file };
        config._ensure_config_file();
        config
    }

    pub fn get(&self) -> Self {
        self.clone()
    }

    fn _ensure_config_file(&self) {
        if self._config_file.exists() {
            return;
        }

        let data: Configuration = serde_json::from_str(&self._config_file.to_str().unwrap())
            .expect("Failed to convert configuration json :(");
    }
}
