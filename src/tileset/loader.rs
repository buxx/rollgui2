use ahash::AHashMap;

use crate::config;

use super::{source::TileSource, TileMapping};

pub fn from_list(
    source: Vec<(&str, i16, i16, i16)>,
    tile_width: f32,
    tile_height: f32,
) -> TileMapping {
    let mut mapping = AHashMap::new();

    for (id, row_i, col_i, sprites_count) in source {
        let tile_source = TileSource::new(
            tile_width * col_i as f32,
            tile_height * row_i as f32,
            tile_width,
            tile_height,
            sprites_count,
            config::SPRITES_COUNT,
        );
        mapping.insert(id.to_string(), tile_source);
    }

    mapping
}
