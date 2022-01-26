use crate::{engine::zone::state, graphics};

use macroquad::prelude::*;

pub fn scene(graphics: &graphics::Graphics, state: &state::ZoneState, tick_i: i16) {
    let map = &state.map;
    let tiles = &state.map.tiles;
    let player = &state.player;

    // TODO : draw only visible tiles
    // Draw zone tiles
    for (row_i, row) in tiles.iter().enumerate() {
        for (col_i, tile_id) in row.iter().enumerate() {
            if tile_id == "UNKNOWN" || tile_id == "NOTHING" {
                continue;
            }

            let foreground_rotation = if tile_id == "DIRT" {
                0.
            } else {
                -state.player.display_rotation.to_radians()
            };

            let dest_x = col_i as f32 * graphics.tile_width;
            let dest_y = row_i as f32 * graphics.tile_height;
            graphics.draw_tile_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                tile_id,
                Some(&state.map.background_tile_id),
                tick_i,
                0.,
                foreground_rotation,
            );
        }
    }

    // Draw player
    let dest_x = player.display_x;
    let dest_y = player.display_y;
    graphics.draw_tile_in_camera(
        map.concrete_width,
        map.concrete_height,
        dest_x,
        dest_y,
        "CHARACTER",
        None,
        tick_i,
        0.,
        -player.display_rotation.to_radians(),
    );
}
