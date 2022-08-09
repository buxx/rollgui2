use crate::message;

pub mod dead;
pub mod description;
pub mod error;
pub mod load_description;
pub mod load_zone;
pub mod root;
pub mod zone;

pub trait Engine {
    fn tick(&mut self) -> Vec<message::MainMessage>;
}
