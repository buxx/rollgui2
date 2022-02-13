use crate::graphics;
use macroquad::prelude::*;

const BUTTON_X: f32 = 960.;
const BUTTON_Y: f32 = 192.;
pub const BUTTON_WIDTH: f32 = 96.;
pub const BUTTON_HEIGHT: f32 = 64.;

pub fn draw_quick_action_button(
    graphics: &graphics::Graphics,
    active: bool,
    tile_id1: &str,
    tile_id2: &str,
    draw_x: f32,
    draw_y: f32,
    tick_i: i16,
) -> bool {
    let draw_end_x = draw_x + BUTTON_WIDTH;
    let draw_end_y = draw_y + BUTTON_HEIGHT;

    let source_x = BUTTON_X;
    let source_y = BUTTON_Y;

    // Draw background
    draw_texture_ex(
        graphics.tileset_texture,
        draw_x,
        draw_y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(source_x, source_y, BUTTON_WIDTH, BUTTON_HEIGHT)),
            ..Default::default()
        },
    );

    // Draw tile 1
    let tile1_source = graphics
        .tiles_mapping
        .get(tile_id1)
        .expect(&format!("Tile id {} is unknown", tile_id1));
    let tile1_source_rect = tile1_source.to_rect(tick_i);

    let dest_size_x = graphics.tile_width * 1.5;
    let dest_size_y = graphics.tile_height * 1.5;

    let tile1_params = DrawTextureParams {
        dest_size: Some(Vec2::new(dest_size_x, dest_size_y)),
        source: Some(tile1_source_rect),
        ..Default::default()
    };

    let tile_draw_x = draw_x + 5.;
    let tile_draw_y = draw_y + 5.;

    draw_texture_ex(
        graphics.tileset_texture,
        tile_draw_x,
        tile_draw_y,
        WHITE,
        tile1_params,
    );

    // Draw tile 2
    let tile2_source = graphics
        .tiles_mapping
        .get(tile_id2)
        .expect(&format!("Tile id {} is unknown", tile_id2));
    let tile2_source_rect = tile2_source.to_rect(tick_i);

    let dest_size_x = graphics.tile_width * 1.5;
    let dest_size_y = graphics.tile_height * 1.5;

    let tile2_params = DrawTextureParams {
        dest_size: Some(Vec2::new(dest_size_x, dest_size_y)),
        source: Some(tile2_source_rect),
        ..Default::default()
    };

    let tile_draw_x = draw_x + 50.;
    let tile_draw_y = draw_y + 5.;

    draw_texture_ex(
        graphics.tileset_texture,
        tile_draw_x,
        tile_draw_y,
        WHITE,
        tile2_params,
    );

    if active {
        draw_rectangle_lines(draw_x, draw_y, BUTTON_WIDTH, BUTTON_HEIGHT, 2.0, BLUE);
    }

    let (mouse_x, mouse_y) = mouse_position();
    mouse_x >= draw_x && mouse_x <= draw_end_x && mouse_y >= draw_y && mouse_y <= draw_end_y
}
