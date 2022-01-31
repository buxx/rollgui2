use crate::graphics;
use macroquad::prelude::*;

const TOP_PANEL_X: f32 = 960.;
const TOP_PANEL_Y: f32 = 256.;
const TOP_PANEL_WIDTH: f32 = 250.;
const TOP_PANEL_HEIGHT: f32 = 42.;
const MIDDLE_PANEL_X: f32 = 960.;
const MIDDLE_PANEL_Y: f32 = 298.;
const MIDDLE_PANEL_WIDTH: f32 = 250.;
const MIDDLE_PANEL_HEIGHT: f32 = 165.;
const BOTTOM_PANEL_X: f32 = 960.;
const BOTTOM_PANEL_Y: f32 = 464.;
const BOTTOM_PANEL_WIDTH: f32 = 250.;
const BOTTOM_PANEL_HEIGHT: f32 = 42.;

pub fn draw_panel_background(graphics: &graphics::Graphics) {
    draw_texture_ex(
        graphics.tileset_texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                TOP_PANEL_X,
                TOP_PANEL_Y,
                TOP_PANEL_WIDTH,
                TOP_PANEL_HEIGHT,
            )),
            ..Default::default()
        },
    );
    let height_count = (screen_height() / MIDDLE_PANEL_HEIGHT as f32) as i32 + 1;
    for i in 0..height_count {
        draw_texture_ex(
            graphics.tileset_texture,
            0.,
            TOP_PANEL_HEIGHT + (MIDDLE_PANEL_HEIGHT as f32 * i as f32) as f32,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    MIDDLE_PANEL_X,
                    MIDDLE_PANEL_Y,
                    MIDDLE_PANEL_WIDTH,
                    MIDDLE_PANEL_HEIGHT,
                )),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        graphics.tileset_texture,
        0.,
        screen_height() - BOTTOM_PANEL_HEIGHT as f32,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                BOTTOM_PANEL_X,
                BOTTOM_PANEL_Y,
                BOTTOM_PANEL_WIDTH,
                BOTTOM_PANEL_HEIGHT,
            )),
            ..Default::default()
        },
    );
}
