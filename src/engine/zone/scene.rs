use crate::{engine::zone::state, graphics};

use macroquad::prelude::*;

pub fn scene(graphics: &graphics::Graphics, state: &state::ZoneState) {
    let tiles = &state.map.tiles;

    for (row_i, row) in tiles.iter().enumerate() {
        for (col_i, tile_id) in row.iter().enumerate() {
            if tile_id == "UNKNOWN" || tile_id == "NOTHING" {
                continue;
            }

            let tile_source = graphics
                .tiles_mapping
                .get(tile_id)
                .expect(&format!("Tile id {} is unknown", tile_id));
            let tile_source_rect = tile_source.to_rect(None);
            // FIXME from config or else
            let dest_x: f32 = row_i as f32 * 32.;
            let dest_y: f32 = col_i as f32 * 32.;
            draw_texture_ex(
                graphics.tileset_texture,
                dest_x,
                dest_y,
                WHITE,
                DrawTextureParams {
                    source: Some(tile_source_rect),
                    ..Default::default()
                },
            );
        }
    }
}
