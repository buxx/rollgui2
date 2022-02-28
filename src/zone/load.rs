use std::collections::HashMap;

use super::{map, ZoneMapTiles};
use crate::{entity, zone};

pub fn from_txt_map(
    source: &str,
    tiles_definitions: Vec<entity::tile::Tile>,
    tiles_width: f32,
    tiles_height: f32,
    default_tile_id: &str,
) -> Result<map::ZoneMap, String> {
    let mapping: HashMap<char, String> = tiles_definitions
        .iter()
        .map(|tile| (tile.char, tile.id.clone()))
        .collect();
    let tiles_as_txt = match source.split("::GEO\n").nth(1) {
        Some(tiles_as_txt) => tiles_as_txt,
        None => {
            return Err(format!(
                "Error when extracting zone source, ::GEO tag not found"
            ))
        }
    };
    let mut tiles: ZoneMapTiles = vec![];
    for txt_row in tiles_as_txt.lines() {
        let mut row_tiles = vec![];
        for tile_char in txt_row.chars() {
            let tile_id = mapping
                .get(&tile_char)
                .unwrap_or(&"UNKNOWN".to_string())
                .clone();
            row_tiles.push(tile_id.to_string());
        }
        tiles.push(row_tiles);
    }
    Ok(zone::map::ZoneMap::new(
        tiles_definitions,
        tiles,
        // FIXME BS NOW : don't hardcode that
        default_tile_id,
        tiles_width,
        tiles_height,
    ))
}
