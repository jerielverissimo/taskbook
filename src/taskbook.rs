use super::storage::Storage;

#[derive(Debug)]
pub struct Taskbook {
    _storage: Storage,
}

impl Taskbook {
    pub fn new() -> Self {
        Taskbook {
            _storage: Storage::new(),
        }
    }

    //fn _archive(&self) {
    //self._storage.get_archive()
    /*}*/
}
