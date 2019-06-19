use super::note::Note;
use super::task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Item {
    TypeTask(Task),
    TypeNote(Note),
}
