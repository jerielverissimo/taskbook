use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &'static str = "data/config.json";

#[derive(Serialize, Deserialize, Debug)]
struct Default {
    #[serde(rename = "taskbookDerictory")]
    taskbook_derictory: String,
    #[serde(rename = "displayCompleteTasks")]
    display_complete_tasks: bool,
    #[serde(rename = "displayProgressOverview")]
    display_progress_overview: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    default: Default,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    configuration: Configuration,
}

#[derive(Clone, Debug)]
pub struct Config {
    _config_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut _config_file = PathBuf::from(env::home_dir().expect("Failed to get home dir :("));
        _config_file = _config_file.join(".taskbook/taskbook.json");

        let config = Config { _config_file };
        config._ensure_config_file();
        config
    }

    pub fn get(&self) -> Options {
        let mut data = File::open(Path::new(DEFAULT_CONFIG)).unwrap();
        let mut contents = String::new();
        data.read_to_string(&mut contents).unwrap();

        let mut config: Options =
            serde_json::from_str(&contents).expect("Failed to convert configuration json :(");

        if config
            .configuration
            .default
            .taskbook_derictory
            .starts_with("~")
        {
            config.configuration.default.taskbook_derictory =
                self._formart_taskbook_dir(config.configuration.default.taskbook_derictory);
        }

        config
    }

    fn _formart_taskbook_dir(&self, path: String) -> String {
        env::home_dir().unwrap().to_str().unwrap().to_string()
    }

    fn _ensure_config_file(&self) {
        if self._config_file.exists() {
            return;
        }

        let mut data = File::open(Path::new(DEFAULT_CONFIG)).unwrap();
        let mut contents = String::new();
        data.read_to_string(&mut contents).unwrap();

        let mut file = File::create(&self._config_file).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        println!("{}", contents);
    }
}
