use crate::graphics;
use macroquad::prelude::*;

const ZOOM_BUTTON_X: f32 = 960.;
const ZOOM_BUTTON_Y: f32 = 512.;
const ZOOM_BUTTON_WIDTH: f32 = 64.;
const ZOOM_BUTTON_HEIGHT: f32 = 64.;

pub fn draw_zoom_button(graphics: &graphics::Graphics) -> bool {
    let draw_start_x = screen_width() - ZOOM_BUTTON_WIDTH as f32;
    let draw_end_x = draw_start_x + ZOOM_BUTTON_WIDTH;
    let draw_start_y = 0.;
    let draw_end_y = draw_start_y + ZOOM_BUTTON_HEIGHT;

    draw_texture_ex(
        graphics.tileset_texture,
        draw_start_x,
        0.,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                ZOOM_BUTTON_X,
                ZOOM_BUTTON_Y,
                ZOOM_BUTTON_WIDTH,
                ZOOM_BUTTON_HEIGHT,
            )),
            ..Default::default()
        },
    );

    let (mouse_x, mouse_y) = mouse_position();
    mouse_x >= draw_start_x
        && mouse_x <= draw_end_x
        && mouse_y >= draw_start_y
        && mouse_y <= draw_end_y
}
