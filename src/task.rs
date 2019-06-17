use super::item::Item;
use super::options::Options;

use chrono::{DateTime, Local};

pub struct Task {
    pub is_complete: bool,
    pub in_progress: bool,
    pub is_starred: bool,
    pub priority: u8,
    pub description: String,
    pub boards: Vec<String>,
    _id: i32,
    _date: DateTime<Local>,
    _timestamp: i64,
    _is_task: bool,
}

impl Item for Task {
    fn new(options: &Options) -> Self {
        Task {
            is_complete: options.is_complete | false,
            in_progress: options.in_progress | false,
            is_starred: options.is_starred | false,
            priority: options.priority | 1,
            _is_task: true,
            _id: options.id,
            _date: Local::now(),
            _timestamp: Local::now().timestamp(),
            description: options.description.clone(),
            boards: Vec::new(),
        }
    }
}
