use super::options::Options;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub description: String,
    pub is_starred: bool,
    pub boards: Vec<String>,
    _id: i32,
    _date: DateTime<Local>,
    _timestamp: i64,
    _is_task: bool,
}

impl Note {
    fn new(options: &Options) -> Self {
        Note {
            _is_task: false,
            _id: options.id,
            _date: Local::now(),
            _timestamp: Local::now().timestamp(),
            description: options.description.clone(),
            is_starred: options.is_starred | false,
            boards: Vec::new(),
        }
    }
}
