use super::note::Note;
use super::task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Items {
    TaskItem(Task),
    NoteItem(Note),
}
