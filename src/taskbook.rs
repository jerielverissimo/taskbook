use super::storage::Storage;

#[derive(Debug)]
pub struct Taskbook {
    _storage: Storage,
}

impl Taskbook {
    pub fn new() -> Self {
        let task = Taskbook {
            _storage: Storage::new(),
        };
        let res = task._storage.get();
        println!("{:#?}", res);
        task
    }

    //fn _archive(&self) {
    //self._storage.get_archive()
    /*}*/
}
