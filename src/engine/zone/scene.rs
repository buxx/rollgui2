use crate::{engine::zone::state, graphics};

use macroquad::prelude::*;

pub fn scene(graphics: &graphics::Graphics, state: &state::ZoneState, tick_i: i16) {
    let map = &state.map;
    let tiles = &state.map.tiles;
    let player_display = &state.player_display;

    // TODO : draw only visible tiles
    // Draw zone tiles
    for (row_i, row) in tiles.iter().enumerate() {
        for (col_i, tile_id) in row.iter().enumerate() {
            if tile_id == "UNKNOWN" || tile_id == "NOTHING" {
                continue;
            }

            // TODO : Identify tiles to keep in front of
            let foreground_rotation = if tile_id == "DIRT" {
                0.
            } else {
                -player_display.rotation.to_radians()
            };

            // Tile destination in the concrete display
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
                None,
                Some(DrawTextureParams {
                    rotation: foreground_rotation,
                    ..Default::default()
                }),
            );
        }
    }

    // Draw player
    let character_tile_id = if state.player_display.moving {
        "CHARACTER_RUNNING"
    } else {
        "CHARACTER"
    };
    graphics.draw_tile_in_camera(
        map.concrete_width,
        map.concrete_height,
        player_display.position.x,
        player_display.position.y,
        character_tile_id,
        None,
        tick_i,
        None,
        Some(DrawTextureParams {
            // Player is always facing the direction of the camera
            rotation: -player_display.rotation.to_radians(),
            // Player pivot must be adapted to follow the camera
            pivot: Some(Vec2::new(
                (player_display.position.x) / map.concrete_width,
                -((player_display.position.y) / map.concrete_height),
            )),
            ..Default::default()
        }),
    );
}
