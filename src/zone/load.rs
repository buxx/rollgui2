use std::collections::HashMap;

use super::{map, ZoneMapTiles};
use crate::zone;

pub fn from_txt_map(source: &str) -> Result<map::ZoneMap, String> {
    let mapping: HashMap<char, &str> = [
        (' ', "NOTHING"),
        ('⡩', "SAND"),
        ('ʛ', "DRY_BUSH"),
        ('#', "ROCK"),
        ('~', "SEA_WATER"),
        ('܄', "SHORT_GRASS"),
        ('፨', "ROCKY_GROUND"),
        ('؛', "HIGH_GRASS"),
        ('⁖', "DIRT"),
        ('߉', "LEAF_TREE"),
        ('ፆ', "TROPICAL_TREE"),
        ('آ', "DEAD_TREE"),
        ('ގ', "FRESH_WATER_TILE"),
        ('c', "COPPER_DEPOSIT"),
        ('t', "TIN_DEPOSIT"),
        ('i', "IRON_DEPOSIT"),
    ]
    .to_vec()
    .into_iter()
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
            let tile_id = mapping.get(&tile_char).unwrap_or(&"UNKNOWN");
            row_tiles.push(tile_id.to_string());
        }
        tiles.push(row_tiles);
    }
    Ok(zone::map::ZoneMap::new(tiles))
}
