use super::config::{Config, Options};
use super::item::Item;

use chrono::{Date, Local};

use colored::*;

pub enum Priorites {
    One,
    Two,
    Three,
}

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

    fn _color_boards(&self, boards: Vec<ColoredString>) -> Vec<ColoredString> {
        boards.into_iter().map(|x| x.bright_black()).collect()
    }

    fn _is_board_complete(&self, items: Vec<Box<dyn Item>>) -> bool {
        let (tasks, complete, notes) = self._get_item_stats(items);
        tasks == complete && notes == 0
    }

    fn _get_age(birthday: Date<Local>) -> ColoredString {
        let daytime = 24 * 60 * 60 * 1000;
        let age = (birthday - Local::today()) / daytime;

        if age == chrono::Duration::zero() {
            "".normal()
        } else {
            format!("{}d", age).bright_black()
        }
    }

    fn _get_corralation(&self, items: Vec<Box<dyn Item>>) -> ColoredString {
        let (tasks, complete, _) = self._get_item_stats(items);
        format!("[${}/${}]", tasks, complete).bright_black()
    }

    fn _get_item_stats(&self, items: Vec<Box<dyn Item>>) -> (i32, i32, i32) {
        let (tasks, complete, notes) = (0, 0, 0);

        items.into_iter().for_each(|item| {
            if item.is_task() {
                tasks += 1;
                if item.as_any().is_complete {
                    return complete += 1;
                }
            }
            return notes += 1;
        });
        (tasks, complete, notes)
    }
}
