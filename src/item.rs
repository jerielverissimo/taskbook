use chrono::{DateTime, Local, TimeZone};

use super::board::Board;
use super::options::Options;

pub struct Item {
    pub description: String,
    pub is_starred: bool,
    pub boards: Vec<Board>,
    _id: i32,
    _date: DateTime<Local>,
    _timestamp: i64,
}

impl Item {
    pub fn new(options: &Options) -> Self {
        Item {
            _id: options.id,
            _date: Local::now(),
            _timestamp: Local::now().timestamp(),
            description: options.description.clone(),
            is_starred: options.is_starred | false,
            boards: Vec::new(),
        }
    }
}
