use super::options::Options;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub is_complete: bool,
    pub in_progress: bool,
    pub starred: bool,
    pub priority: u8,
    pub description: String,
    pub boards: Vec<String>,
    _id: i32,
    _date: DateTime<Local>,
    _timestamp: i64,
    _is_task: bool,
}

impl Task {
    pub fn new(options: &Options) -> Self {
        Task {
            is_complete: options.is_complete | false,
            in_progress: options.in_progress | false,
            starred: options.is_starred | false,
            priority: if options.priority == 0 {
                1
            } else {
                options.priority
            },
            _is_task: true,
            _id: options.id,
            _date: Local::now(),
            _timestamp: Local::now().timestamp(),
            description: options.description.clone(),
            boards: Vec::new(),
        }
    }
}

impl super::item::Item for Task {
    fn is_starred(&self) -> bool {
        self.starred
    }
}
