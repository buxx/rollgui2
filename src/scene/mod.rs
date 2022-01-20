pub mod root;

use crate::message;

pub trait Scene {
    fn run(&mut self) -> Option<message::MainMessage>;
}
