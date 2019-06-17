use super::options::Options;

pub trait Item {
    fn new(options: &Options) -> Self;
}
