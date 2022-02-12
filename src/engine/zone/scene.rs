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
                None,
            );
        }
    }

    // TODO : draw only visible tiles
    // Draw builds tiles
    for (_, build) in &state.builds {
        let dest_x = build.col_i as f32 * graphics.tile_width;
        let dest_y = build.row_i as f32 * graphics.tile_height;

        // TODO : optimize by compute each stuff_id / tile_id a zone creation
        let tile_id = graphics.find_tile_id_from_classes(&build.classes);
        graphics.draw_tile_in_camera(
            map.concrete_width,
            map.concrete_height,
            dest_x,
            dest_y,
            &tile_id,
            None,
            tick_i,
            None,
            None,
        );
    }

    // TODO : draw only visible tiles
    // Draw resource tiles
    for resource in state.resources.values() {
        let dest_x = resource.zone_col_i as f32 * graphics.tile_width;
        let dest_y = resource.zone_row_i as f32 * graphics.tile_height;

        graphics.draw_tile_in_camera(
            map.concrete_width,
            map.concrete_height,
            dest_x,
            dest_y,
            &resource.id,
            None,
            tick_i,
            None,
            None,
        );
    }

    // TODO : draw only visible tiles
    // Draw stuff tiles
    for (_, stuff) in &state.stuffs {
        let dest_x = stuff.zone_col_i as f32 * graphics.tile_width;
        let dest_y = stuff.zone_row_i as f32 * graphics.tile_height;

        // TODO : optimize by compute each stuff_id / tile_id a zone creation
        let tile_id = graphics.find_tile_id_from_classes(&stuff.classes);
        graphics.draw_tile_in_camera(
            map.concrete_width,
            map.concrete_height,
            dest_x,
            dest_y,
            &tile_id,
            None,
            tick_i,
            None,
            None,
        );
    }

    for character in state.characters.values() {
        let dest_x = character.zone_col_i as f32 * graphics.tile_width;
        let dest_y = character.zone_row_i as f32 * graphics.tile_height;

        graphics.draw_tile_in_camera(
            map.concrete_width,
            map.concrete_height,
            dest_x,
            dest_y,
            "CHARACTER",
            None,
            tick_i,
            None,
            None,
        );
    }

    // Draw player
    let character_tile_id = match state.player_display.running {
        Some(super::PlayerRunning::Top) => "CHARACTER_RUNNING_TOP",
        Some(super::PlayerRunning::Down) => "CHARACTER_RUNNING_DOWN",
        Some(super::PlayerRunning::Left) => "CHARACTER_RUNNING_LEFT",
        Some(super::PlayerRunning::Right) => "CHARACTER_RUNNING_RIGHT",
        None => "CHARACTER",
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
        None,
    );
}
