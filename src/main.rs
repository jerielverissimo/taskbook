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
    let conf = config::Config::new();

    println!("{:?}", conf);
    println!("{:?}", taskbook);
}
