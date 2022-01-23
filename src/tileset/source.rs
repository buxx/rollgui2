#[derive(Clone)]
pub struct TileSource {
    x: i16,
    y: i16,
    width: f32,
    height: f32,
    sprites_count: i16,
}

impl TileSource {
    pub fn new(x: i16, y: i16, width: f32, height: f32, sprites_count: i16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            sprites_count,
        }
    }
}
