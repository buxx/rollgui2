use crate::{engine::zone::state, graphics};

use macroquad::prelude::*;

pub fn scene(graphics: &graphics::Graphics, _state: &state::ZoneState) {
    let tree_source: Rect = Rect::new(32. * 0., 32. * 2., 32., 32.);

    for x in 0..255 {
        for y in 0..255 {
            draw_texture_ex(
                graphics.tileset_texture,
                32. * x as f32,
                32. * y as f32,
                WHITE,
                DrawTextureParams {
                    source: Some(tree_source),
                    ..Default::default()
                },
            );
        }
    }
}
