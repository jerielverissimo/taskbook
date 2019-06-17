use super::config::{Config, Options};

use colored::*;

pub struct Render {
    _config: Config,
}

impl Render {
    pub fn new(config: Config) -> Self {
        Render { _config: config }
    }
    pub fn get_configuration(&self) -> Options {
        self._config.get()
    }

    fn _color_boards(boards: Vec<ColoredString>) -> Vec<ColoredString> {
        boards.into_iter().map(|x| x.bright_black()).collect()
    }
}
