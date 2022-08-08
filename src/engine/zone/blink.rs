use super::ZoneEngine;
use macroquad::prelude::*;

const ICON_WIDTH: f32 = 24.0;
const ICON_HEIGHT: f32 = 24.0;

impl ZoneEngine {
    pub fn draw_blinking_icons(&mut self) {
        let mut indexes_to_removes: Vec<usize> = vec![];

        let start_y = screen_width() - ICON_WIDTH * self.blinking_icons.len() as f32;
        for (i, blinking_icon) in self.blinking_icons.iter_mut().enumerate() {
            if blinking_icon.update(self.frame_i) {
                indexes_to_removes.push(i);
            }

            let dest = Vec2::new(
                start_y + ICON_WIDTH * i as f32,
                screen_height() - ICON_HEIGHT,
            );
            draw_rectangle(dest.x, dest.y, ICON_WIDTH, ICON_HEIGHT, LIGHTGRAY);

            if blinking_icon.visible() {
                draw_texture_ex(
                    self.graphics.tileset_texture,
                    dest.x,
                    dest.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(ICON_WIDTH, ICON_HEIGHT)),
                        source: Some(blinking_icon.source()),
                        ..Default::default()
                    },
                );
            }
        }

        // Remove finished blinking icons
        indexes_to_removes.sort();
        indexes_to_removes.reverse();
        for index_to_remove in indexes_to_removes {
            self.blinking_icons.remove(index_to_remove);
        }
    }
}
