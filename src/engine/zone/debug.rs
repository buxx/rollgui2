use macroquad::prelude::*;

use super::{util::in_area, ZoneEngine};

pub struct DebugInfo {
    tile_count: i32,
    resource_count: i32,
    stuff_count: i32,
    character_count: i32,
    build_count: i32,
    fps: i32,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            tile_count: 0,
            resource_count: 0,
            stuff_count: 0,
            character_count: 0,
            build_count: 0,
            fps: 0,
        }
    }

    pub fn incr_zone_tile_count(&mut self) {
        self.tile_count += 1;
    }

    pub fn incr_resource_count(&mut self) {
        self.resource_count += 1;
    }

    pub fn incr_stuff_count(&mut self) {
        self.stuff_count += 1;
    }

    pub fn incr_character_count(&mut self) {
        self.character_count += 1;
    }

    pub fn incr_build_count(&mut self) {
        self.build_count += 1;
    }

    pub fn set_fps(&mut self, fps: i32) {
        self.fps = fps
    }

    pub fn reset(&mut self) {
        self.tile_count = 0;
        self.resource_count = 0;
        self.stuff_count = 0;
        self.character_count = 0;
        self.build_count = 0;
        self.fps = 0;
    }

    pub fn to_string(&self) -> String {
        format!(
            "FPS:{},T:{},R:{},S:{},C:{},B:{}",
            self.fps,
            self.tile_count,
            self.resource_count,
            self.stuff_count,
            self.character_count,
            self.build_count
        )
    }
}

impl ZoneEngine {
    pub fn draw_zone_debug(&self, draw_area: ((i32, i32), (i32, i32))) {
        if !self.display_debug_info {
            return;
        }

        self.draw_tiles_coordinates(draw_area);
    }

    fn draw_tiles_coordinates(&self, draw_area: ((i32, i32), (i32, i32))) {
        let text_size = 12.0;
        let zoom_factor = self.zoom_mode.factor();
        let text_y_adjust = (self.graphics.tile_height * zoom_factor) - (text_size / 2.0) - 1.0;
        for row_i in 0..self.state.map.tiles.len() {
            for col_i in 0..self.state.map.tiles.first().unwrap().len() {
                if !in_area(row_i as i32, col_i as i32, &draw_area) {
                    continue;
                }

                let screen_position =
                    self.zone_position_to_screen_position(row_i as f32, col_i as f32);

                draw_text(
                    &format!("{row_i}.{col_i}"),
                    screen_position.x,
                    screen_position.y - text_y_adjust,
                    12.0,
                    YELLOW,
                );

                draw_rectangle_lines(
                    screen_position.x,
                    screen_position.y,
                    self.graphics.tile_width * zoom_factor,
                    self.graphics.tile_height * zoom_factor,
                    1.0,
                    YELLOW,
                )
            }
        }
    }
}
