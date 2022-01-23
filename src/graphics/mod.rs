use macroquad::prelude::*;

use crate::tileset;

#[derive(Clone)]
pub struct Graphics {
    pub tileset_texture: Texture2D,
    pub tiles_mapping: tileset::TileMapping,
}

impl Graphics {
    pub fn new(tileset_texture: Texture2D, tiles_mapping: tileset::TileMapping) -> Self {
        Self {
            tileset_texture,
            tiles_mapping,
        }
    }
}
