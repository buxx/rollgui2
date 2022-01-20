pub mod root;

use crate::message;

pub trait Engine {
    fn run(&mut self) -> Option<message::MainMessage>;
}
