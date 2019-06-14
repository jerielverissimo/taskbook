use super::item::Item;
use super::options::Options;

pub struct Task {
    pub item: Item,
    pub is_complete: bool,
    pub in_progress: bool,
    pub is_starred: bool,
    pub priority: u8,
    _is_task: bool,
}

impl Task {
    pub fn new(options: &Options) -> Self {
        Task {
            item: Item::new(&options),
            is_complete: options.is_complete | false,
            in_progress: options.in_progress | false,
            is_starred: options.is_starred | false,
            priority: options.priority | 1,
            _is_task: true,
        }
    }
}
