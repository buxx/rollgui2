use crate::{engine::zone::state, graphics, util::get_text_center};

use macroquad::prelude::*;

use super::{debug::DebugInfo, util::in_area, ZoneEngine};

pub fn scene(
    graphics: &graphics::Graphics,
    state: &state::ZoneState,
    tick_i: i16,
    draw_area: ((i32, i32), (i32, i32)),
) -> DebugInfo {
    let mut display_counter = DebugInfo::new();
    let map = &state.map;
    let tiles = &state.map.tiles;
    let player_display = &state.player_display;

    // Draw zone tiles
    for (row_i, row) in tiles.iter().enumerate() {
        for (col_i, tile_id) in row.iter().enumerate() {
            if tile_id == "UNKNOWN"
                || tile_id == "NOTHING"
                || !in_area(row_i as i32, col_i as i32, &draw_area)
            {
                continue;
            }

            // Tile destination in the concrete display
            let dest_x = col_i as f32 * graphics.tile_width;
            let dest_y = row_i as f32 * graphics.tile_height;

            display_counter.incr_zone_tile_count();
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

    // Draw builds tiles
    for (_, build) in &state.builds {
        let dest_x = build.col_i as f32 * graphics.tile_width;
        let dest_y = build.row_i as f32 * graphics.tile_height;

        if !in_area(build.row_i, build.col_i, &draw_area) {
            continue;
        }

        // TODO : optimize by compute each stuff_id / tile_id a zone creation
        let tile_id = graphics.find_tile_id_from_classes(&build.classes);
        display_counter.incr_build_count();
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

        if build.under_construction {
            graphics.draw_tile_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                "LITTLE_SHOVEL",
                None,
                tick_i,
                None,
                None,
            );
        }
    }

    // Draw resource tiles
    for resources in state.resources.values() {
        for resource in resources {
            if !in_area(resource.zone_row_i, resource.zone_col_i, &draw_area) {
                continue;
            }

            let dest_x = resource.zone_col_i as f32 * graphics.tile_width;
            let dest_y = resource.zone_row_i as f32 * graphics.tile_height;

            display_counter.incr_resource_count();
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
    }

    // Draw stuff tiles
    for (_, stuff) in &state.stuffs {
        if !in_area(stuff.zone_row_i, stuff.zone_col_i, &draw_area) {
            continue;
        }

        let dest_x = stuff.zone_col_i as f32 * graphics.tile_width;
        let dest_y = stuff.zone_row_i as f32 * graphics.tile_height;

        // TODO : optimize by compute each stuff_id / tile_id a zone creation
        // TODO : clone here is bad performance
        let classes = vec![stuff.classes.clone(), vec![stuff.stuff_id.clone()]]
            .into_iter()
            .flatten()
            .collect();

        let tile_id = graphics.find_tile_id_from_classes(&classes);
        display_counter.incr_stuff_count();
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
        if character.id == state.player.id
            || !in_area(character.zone_row_i, character.zone_col_i, &draw_area)
        {
            continue;
        }

        let dest_x = character.zone_col_i as f32 * graphics.tile_width;
        let dest_y = character.zone_row_i as f32 * graphics.tile_height;

        display_counter.incr_character_count();
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
    display_counter.incr_character_count();
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

    display_counter
}

impl ZoneEngine {
    pub fn draw_characters_names(&self, draw_area: ((i32, i32), (i32, i32))) {
        let zoom_factor = self.zoom_mode.factor();
        let half_tile_width = (self.graphics.tile_width * zoom_factor) / 2.0;
        let tile_height = self.graphics.tile_width * zoom_factor;
        let font_size: u16 = 24;
        let font_scale = 1.0;

        for character in self.state.characters.values() {
            if !in_area(character.zone_row_i, character.zone_col_i, &draw_area) {
                continue;
            }

            let text_center = get_text_center(&character.name, None, font_size, font_scale, 0.);
            let dest_row_i = character.zone_row_i as f32;
            let dest_col_i = character.zone_col_i as f32;
            let screen_position = self.zone_position_to_screen_position(dest_row_i, dest_col_i);
            draw_text_ex(
                &character.name,
                screen_position.x + half_tile_width - text_center.x,
                screen_position.y - tile_height - text_center.y - (font_size as f32 / 2.0),
                TextParams {
                    font_size: font_size,
                    font_scale: font_scale,
                    color: RED,
                    ..Default::default()
                },
            )
        }
    }
}
