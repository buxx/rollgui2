use macroquad::prelude::*;

#[derive(Clone)]
pub struct TileSource {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    sprites_count: i16,
}

impl TileSource {
    pub fn new(x: f32, y: f32, width: f32, height: f32, sprites_count: i16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            sprites_count,
        }
    }

    pub fn to_rect(&self, _sprite_index: Option<usize>) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
