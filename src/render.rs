use super::config::{Config, Options};
use super::item::{Item, Items};

use chrono::{Date, DateTime, Local};

use colored::*;

#[derive(Debug)]
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

    fn _is_board_complete(&self, items: Vec<&Items>) -> bool {
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

    fn _get_correlation(&self, items: Vec<&Items>) -> ColoredString {
        let (tasks, complete, _) = self._get_item_stats(items);
        format!("[${}/${}]", tasks, complete).bright_black()
    }

    fn _get_item_stats(&self, items: Vec<&Items>) -> (i32, i32, i32) {
        let (mut tasks, mut complete, mut notes) = (0, 0, 0);

        items.into_iter().for_each(|item| match item {
            Items::TaskItem(task) => {
                tasks += 1;
                if task.is_complete {
                    complete += 1;
                }
            }
            Items::NoteItem(_) => notes += 1,
        });
        (tasks, complete, notes)
    }

    fn _get_star(&self, item: &Items) -> ColoredString {
        item.do_action(|i: Box<&dyn Item>| {
            if i.is_starred() {
                return "★".yellow();
            } else {
                return "".normal();
            }
        })
    }

    fn _build_title(
        &self,
        key: DateTime<Local>,
        items: Vec<&Items>,
    ) -> (ColoredString, ColoredString) {
        let title: ColoredString = if key.to_string() == Local::now().to_string() {
            ColoredString::from(
                format!(
                    "{} {}",
                    key.to_string().underline(),
                    "'[Today]'".bright_black()
                )
                .as_str(),
            )
        } else {
            key.to_string().underline()
        };
        let correlation = self._get_correlation(items);

        (title, correlation)
    }
}
