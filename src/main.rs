mod board;
mod config;
mod item;
mod note;
mod options;
mod render;
mod storage;
mod task;
mod taskbook;

fn main() {
    let taskbook = taskbook::Taskbook::new();

    println!("{:#?}", taskbook);
}
