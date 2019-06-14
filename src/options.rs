#[derive(Clone)]
pub struct Options {
    pub id: i32,
    pub description: String,
    pub is_starred: bool,
    pub boards: Vec<super::board::Board>,
    pub is_complete: bool,
    pub in_progress: bool,
    pub priority: u8,
}
