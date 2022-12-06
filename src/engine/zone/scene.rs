use crate::{engine::zone::state, graphics, util::get_text_center};

use macroquad::prelude::*;

use super::{debug::DebugInfo, util::in_area, ZoneEngine};

impl ZoneEngine {
    pub fn scene(&mut self, draw_area: ((i32, i32), (i32, i32))) {
        let mut display_counter = DebugInfo::new();
        let map = &self.state.map;
        let tiles = &self.state.map.tiles;
        let player_display = &self.state.player_display;

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
                let dest_x = col_i as f32 * self.graphics.tile_width;
                let dest_y = row_i as f32 * self.graphics.tile_height;

                display_counter.incr_zone_tile_count();
                self.graphics.draw_tile_in_camera(
                    map.concrete_width,
                    map.concrete_height,
                    dest_x,
                    dest_y,
                    tile_id,
                    Some(&self.state.map.background_tile_id),
                    self.tick_i,
                    None,
                    None,
                );
            }
        }

        // Draw builds tiles
        for (_, build) in &self.state.builds {
            let dest_x = build.col_i as f32 * self.graphics.tile_width;
            let dest_y = build.row_i as f32 * self.graphics.tile_height;

            if !in_area(build.row_i, build.col_i, &draw_area) {
                continue;
            }

            // TODO : optimize by compute each stuff_id / tile_id a zone creation
            let tile_id = self.graphics.find_tile_id_from_classes(&build.classes);
            display_counter.incr_build_count();
            self.graphics.draw_tile_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                &tile_id,
                None,
                self.tick_i,
                None,
                None,
            );

            if build.under_construction {
                self.graphics.draw_tile_in_camera(
                    map.concrete_width,
                    map.concrete_height,
                    dest_x,
                    dest_y,
                    "LITTLE_SHOVEL",
                    None,
                    self.tick_i,
                    None,
                    None,
                );
            }
        }

        // Draw resource tiles
        for resources in self.state.resources.values() {
            for resource in resources {
                if !in_area(resource.zone_row_i, resource.zone_col_i, &draw_area) {
                    continue;
                }

                let dest_x = resource.zone_col_i as f32 * self.graphics.tile_width;
                let dest_y = resource.zone_row_i as f32 * self.graphics.tile_height;

                display_counter.incr_resource_count();
                self.graphics.draw_tile_in_camera(
                    map.concrete_width,
                    map.concrete_height,
                    dest_x,
                    dest_y,
                    &resource.id,
                    None,
                    self.tick_i,
                    None,
                    None,
                );
            }
        }

        // Draw stuff tiles
        for (_, stuff) in &self.state.stuffs {
            if !in_area(stuff.zone_row_i, stuff.zone_col_i, &draw_area) {
                continue;
            }

            let dest_x = stuff.zone_col_i as f32 * self.graphics.tile_width;
            let dest_y = stuff.zone_row_i as f32 * self.graphics.tile_height;

            // TODO : optimize by compute each stuff_id / tile_id a zone creation
            // TODO : clone here is bad performance
            let classes = vec![stuff.classes.clone(), vec![stuff.stuff_id.clone()]]
                .into_iter()
                .flatten()
                .collect();

            let tile_id = self.graphics.find_tile_id_from_classes(&classes);
            display_counter.incr_stuff_count();
            self.graphics.draw_tile_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                &tile_id,
                None,
                self.tick_i,
                None,
                None,
            );
        }

        for character in self.state.characters.values() {
            if character.id == self.state.player.id
                || !in_area(character.zone_row_i, character.zone_col_i, &draw_area)
            {
                continue;
            }

            let dest_x = character.zone_col_i as f32 * self.graphics.tile_width;
            let dest_y = character.zone_row_i as f32 * self.graphics.tile_height;

            display_counter.incr_character_count();
            self.graphics.draw_character_in_camera(
                map.concrete_width,
                map.concrete_height,
                dest_x,
                dest_y,
                self.tick9_i,
                &character.id,
                &None,
            );
        }

        // Draw player
        self.graphics.draw_character_in_camera(
            map.concrete_width,
            map.concrete_height,
            self.state.player_display.position.x,
            self.state.player_display.position.y,
            self.tick9_i,
            &self.state.player.id,
            &self.state.player_display.running,
        );

        if self.frame_i % 30 == 0 {
            self.debug_info = display_counter;
            self.debug_info.set_fps(get_fps());
        }
    }

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
                screen_position.y - (tile_height * 1.5) - text_center.y - (font_size as f32 / 2.0),
                TextParams {
                    font_size: font_size,
                    font_scale: font_scale,
                    color: Color::new(0.20, 1.0, 01.0, 1.00),
                    ..Default::default()
                },
            )
        }
    }
}
