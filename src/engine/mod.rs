use crate::{graphics, message};

pub mod dead;
pub mod description;
pub mod error;
pub mod load_description;
pub mod load_zone;
pub mod root;
pub mod world;
pub mod zone;

pub trait Engine {
    fn tick(&mut self) -> Vec<message::MainMessage>;
    fn replace_graphics(&mut self, graphics: graphics::Graphics);
    fn signal_illustration_loaded(&mut self, illustration_name: &str);
}
