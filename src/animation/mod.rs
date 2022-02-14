pub mod drop;
pub mod pop;

use crate::graphics;

pub trait Animation {
    fn update(&mut self, frame_i: i64) -> bool;
    fn draw_in_camera(&self, graphics: &graphics::Graphics);
}
