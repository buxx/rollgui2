use crate::graphics;
use macroquad::prelude::*;

const ZOOM_BUTTON_X: f32 = 960.;
const ZOOM_BUTTON_Y: f32 = 512.;
const ZOOM_BUTTON_WIDTH: f32 = 64.;
const ZOOM_BUTTON_HEIGHT: f32 = 64.;

const RUN_BUTTON_X: f32 = 960.;
const RUN_BUTTON_Y: f32 = 576.;
const RUN_BUTTON_WIDTH: f32 = 64.;
const RUN_BUTTON_HEIGHT: f32 = 64.;

pub fn draw_zoom_button(graphics: &graphics::Graphics, active: bool) -> bool {
    let draw_start_x = screen_width() - ZOOM_BUTTON_WIDTH as f32;
    let draw_end_x = draw_start_x + ZOOM_BUTTON_WIDTH;
    let draw_start_y = 0.;
    let draw_end_y = draw_start_y + ZOOM_BUTTON_HEIGHT;

    let source_x = if active {
        ZOOM_BUTTON_X + ZOOM_BUTTON_WIDTH
    } else {
        ZOOM_BUTTON_X
    };
    let source_y = ZOOM_BUTTON_Y;

    draw_texture_ex(
        graphics.tileset_texture,
        draw_start_x,
        draw_start_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                source_x,
                source_y,
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

pub fn draw_run_button(graphics: &graphics::Graphics, active: bool) -> bool {
    let draw_start_x = screen_width() - RUN_BUTTON_WIDTH as f32;
    let draw_end_x = draw_start_x + RUN_BUTTON_WIDTH;
    let draw_start_y = ZOOM_BUTTON_HEIGHT;
    let draw_end_y = draw_start_y + RUN_BUTTON_HEIGHT;

    let source_x = if active {
        RUN_BUTTON_X + RUN_BUTTON_WIDTH
    } else {
        RUN_BUTTON_X
    };
    let source_y = RUN_BUTTON_Y;

    draw_texture_ex(
        graphics.tileset_texture,
        draw_start_x,
        draw_start_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                source_x,
                source_y,
                RUN_BUTTON_WIDTH,
                RUN_BUTTON_HEIGHT,
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
