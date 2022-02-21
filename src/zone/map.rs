use std::collections::HashMap;

use crate::entity;

use super::ZoneMapTiles;

#[derive(Clone)]
pub struct ZoneMap {
    pub tiles_definitions: HashMap<String, entity::tile::Tile>,
    pub tiles: ZoneMapTiles,
    pub background_tile_id: String,
    pub width: usize,
    pub height: usize,
    // Computed map width according to tiles width
    pub concrete_width: f32,
    // Computed map width according to tiles height
    pub concrete_height: f32,
}

impl ZoneMap {
    pub fn new(
        tiles_definitions: Vec<entity::tile::Tile>,
        tiles: ZoneMapTiles,
        background_tile_id: &str,
        tiles_width: f32,
        tiles_height: f32,
    ) -> Self {
        let map_width_count = tiles
            .iter()
            .map(|row| row.len())
            .max()
            .expect("Unable to determine zone width count") as f32;
        let map_height_count = tiles.len() as f32;
        let concrete_width = map_width_count * tiles_width;
        let concrete_height = map_height_count * tiles_height;
        let tiles_definitions: HashMap<String, entity::tile::Tile> = tiles_definitions
            .into_iter()
            .map(|tile| (tile.id.clone(), tile))
            .collect();

        Self {
            tiles_definitions,
            tiles,
            background_tile_id: background_tile_id.to_string(),
            width: map_width_count as usize,
            height: map_height_count as usize,
            concrete_width,
            concrete_height,
        }
    }

    pub fn replace_tile(&mut self, row_i: i16, col_i: i16, new_tile_id: String) {
        if let Some(row) = self.tiles.get_mut(row_i as usize) {
            if let Some(tile) = row.get_mut(col_i as usize) {
                *tile = new_tile_id;
            }
        }
    }

    pub fn tile_id(&self, row_i: usize, col_i: usize) -> Option<String> {
        if row_i < 0 || col_i < 0 {
            return None;
        }

        if let Some(row) = self.tiles.get(row_i) {
            if col_i >= row.len() {
                return None;
            }

            return Some(row[col_i].clone());
        }

        None
    }

    pub fn traversable(
        &self,
        row_i: usize,
        col_i: usize,
        transport_mode: &entity::tile::TransportMode,
    ) -> bool {
        if let Some(tile_id) = self.tile_id(row_i, col_i) {
            if let Some(tile) = self.tiles_definitions.get(&tile_id) {
                if let Some(traversable) = tile.traversable.get(transport_mode) {
                    return *traversable;
                }
            }
        };
        false
    }

    pub fn get_speed(
        &self,
        row_i: usize,
        col_i: usize,
        transport_mode: &entity::tile::TransportMode,
    ) -> f32 {
        if let Some(tile_id) = self.tile_id(row_i, col_i) {
            if let Some(tile) = self.tiles_definitions.get(&tile_id) {
                if let Some(hump_type) = tile.hump.get(transport_mode) {
                    return hump_type.to_speed();
                }
            }
        };

        entity::tile::HumpType::Normal.to_speed()
    }
}
