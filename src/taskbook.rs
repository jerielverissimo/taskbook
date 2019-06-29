use super::config::Config;
use super::item::Items;
use super::options::Options;
use super::render::Render;
use super::storage::Storage;
use super::task::Task;

#[derive(Debug)]
pub struct Taskbook {
    _storage: Storage,
    _render: Render,
}

impl Taskbook {
    pub fn new() -> Self {
        let task = Taskbook {
            _storage: Storage::new(),
            _render: Render::new(Config::new()),
        };
        let res = task._storage.get();
        println!("{:#?}", res);
        task
    }

    fn _archive(&self) -> Vec<Items> {
        self._storage.get_archive()
    }

    fn _data(&self) -> Vec<Items> {
        self._storage.get()
    }
}
