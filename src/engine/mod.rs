pub mod root;
pub mod zone;

use crate::message;

pub trait Engine {
    fn run(&mut self) -> Option<message::MainMessage>;
}
