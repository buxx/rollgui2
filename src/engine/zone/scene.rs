use crate::{engine::zone::state, graphics};

use macroquad::prelude::*;

pub fn scene(graphics: &graphics::Graphics, state: &state::ZoneState) {
    let map = &state.map;
    let tiles = &state.map.tiles;

    let zoom_x = (map.concrete_width / screen_width()) * 2.;
    let zoom_y = (map.concrete_height / screen_height()) * 2.;

    set_camera(&Camera2D {
        zoom: Vec2::new(zoom_x, zoom_y),
        offset: Vec2::new(-1.0, -1.0), // FIXME depending from player
        ..Default::default()
    });

    for (row_i, row) in tiles.iter().enumerate() {
        for (col_i, tile_id) in row.iter().enumerate() {
            if tile_id == "UNKNOWN" || tile_id == "NOTHING" {
                continue;
            }

            let dest_x = col_i as f32 * graphics.tile_width;
            let dest_y = row_i as f32 * graphics.tile_height;
            graphics.draw_tile_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                tile_id,
                Some(&state.map.background_tile_id),
            );
        }
    }
}
