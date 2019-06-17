use super::item::Item;
use super::options::Options;

use chrono::{DateTime, Local};

pub struct Note {
    pub description: String,
    pub is_starred: bool,
    pub boards: Vec<String>,
    _id: i32,
    _date: DateTime<Local>,
    _timestamp: i64,
    _is_task: bool,
}

impl Item for Note {
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
