use super::note::Note;
use super::task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Items {
    TaskItem(Task),
    NoteItem(Note),
}

impl Items {
    pub fn do_action<T>(&self, action: impl Fn(Box<&dyn Item>) -> T) -> T {
        match self {
            Items::TaskItem(task) => action(Box::new(task)),
            Items::NoteItem(note) => action(Box::new(note)),
        }
    }
}

pub trait Item {
    fn is_starred(&self) -> bool;
}

