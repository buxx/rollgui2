use macroquad::prelude::*;

use crate::graphics;

pub struct VisibleAnimation {
    source: Rect,
    dest: Vec2,
    displayed_until_frame_i: i64,
}

impl super::Animation for VisibleAnimation {
    fn update(&mut self, frame_i: i64) -> bool {
        frame_i > self.displayed_until_frame_i
    }

    fn draw_in_camera(&self, graphics: &graphics::Graphics) {
        let params = DrawTextureParams {
            source: Some(self.source),
            ..Default::default()
        };
        draw_texture_ex(
            graphics.tileset_texture,
            self.dest.x,
            self.dest.y,
            WHITE,
            params,
        );
    }
}

impl VisibleAnimation {
    pub fn new(source: Rect, dest: Vec2, displayed_until_frame_i: i64) -> Self {
        Self {
            source,
            dest,
            displayed_until_frame_i,
        }
    }
}
