use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct TileSource {
    pub sprites: Vec<Sprite>,
    pub width: f32,
    pub height: f32,
}

impl TileSource {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        real_sprites_count: i16,
        required_sprites_count: i16,
    ) -> Self {
        let mut sprites = vec![];

        for i in 0..required_sprites_count + 1 {
            let sprite_x = if real_sprites_count > i {
                x + (width * i as f32)
            } else {
                x
            };
            let sprite_y = y;
            sprites.push(Sprite {
                x: sprite_x,
                y: sprite_y,
            });
        }

        Self {
            sprites,
            width,
            height,
        }
    }

    pub fn to_rect(&self, sprite_index: i16) -> Rect {
        let sprite = self
            .sprites
            .get(sprite_index as usize)
            .expect(&format!("Sprite index {} out of bounds", sprite_index));
        Rect::new(
            sprite.x + 1.,
            sprite.y + 1.,
            self.width - 2.,
            self.height - 2.,
        )
    }
}
