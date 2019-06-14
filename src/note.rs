use super::item::Item;
use super::options::Options;

pub struct Note {
    pub item: Item,
    _is_task: bool,
}

impl Note {
    pub fn new(options: &Options) -> Self {
        Note {
            item: Item::new(options),
            _is_task: false,
        }
    }
}
