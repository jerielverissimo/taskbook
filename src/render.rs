use super::config::{Config, Options};

use colored::*;

pub struct Render {
    config: Config,
}

impl Render {
    pub fn get_configuration(&self) -> Options {
        self.config.get()
    }

    fn _color_boards(boards: Vec<ColoredString>) -> Vec<ColoredString> {
        boards.into_iter().map(|x| x.bright_black()).collect()
    }
}
