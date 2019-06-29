#[derive(Clone, Debug)]
pub struct Options {
    pub id: i32,
    pub description: String,
    pub is_starred: bool,
    pub boards: Vec<super::board::Board>,
    pub is_complete: bool,
    pub in_progress: bool,
    pub priority: u8,
}

impl Options {
    pub fn new(
        id: i32,
        description: String,
        is_starred: bool,
        boards: Vec<super::board::Board>,
        is_complete: bool,
        in_progress: bool,
        priority: u8,
    ) -> Self {
        Self {
            id,
            description,
            is_starred,
            boards,
            is_complete,
            in_progress,
            priority,
        }
    }
}
