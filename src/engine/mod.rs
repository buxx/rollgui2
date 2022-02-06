pub mod error;
pub mod load_zone;
pub mod root;
pub mod zone;

use crate::message;

pub trait Engine {
    fn init(&mut self) -> Vec<message::MainMessage> {
        vec![]
    }
    fn tick(&mut self) -> Vec<message::MainMessage>;
}
