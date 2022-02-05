pub mod error;
pub mod load_zone;
pub mod root;
pub mod zone;

use crate::message;

pub trait Engine {
    fn run(&mut self) -> Vec<message::MainMessage>;
}
