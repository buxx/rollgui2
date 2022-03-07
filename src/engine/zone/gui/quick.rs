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
    quick_action_key: &Option<char>,
    tick_i: i16,
) -> bool {
    let multiplier = if graphics.draw_as_mobile { 2.0 } else { 1.0 };

    let draw_end_x = draw_x + (BUTTON_WIDTH * multiplier);
    let draw_end_y = draw_y + (BUTTON_HEIGHT * multiplier);

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
            dest_size: Some(Vec2::new(
                BUTTON_WIDTH * multiplier,
                BUTTON_HEIGHT * multiplier,
            )),
            ..Default::default()
        },
    );

    // Draw tile 1
    let tile1_source = graphics
        .tiles_mapping
        .get(tile_id1)
        .expect(&format!("Tile id {} is unknown", tile_id1));
    let tile1_source_rect = tile1_source.to_rect(tick_i);

    let dest_size_x = graphics.tile_width * 1.5 * multiplier;
    let dest_size_y = graphics.tile_height * 1.5 * multiplier;

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

    let dest_size_x = graphics.tile_width * 1.5 * multiplier;
    let dest_size_y = graphics.tile_height * 1.5 * multiplier;

    let tile2_params = DrawTextureParams {
        dest_size: Some(Vec2::new(dest_size_x, dest_size_y)),
        source: Some(tile2_source_rect),
        ..Default::default()
    };

    let tile_draw_x = draw_x + (50. * multiplier);
    let tile_draw_y = draw_y + 5.;

    draw_texture_ex(
        graphics.tileset_texture,
        tile_draw_x,
        tile_draw_y,
        WHITE,
        tile2_params,
    );

    if let Some(quick_action_key_) = quick_action_key {
        draw_circle(draw_x + 5., draw_y + 5., 10., BEIGE);
        draw_text(
            &format!("{}", quick_action_key_),
            draw_x + 1.,
            draw_y + 10.,
            20.0,
            BLACK,
        );
    }

    if active {
        draw_rectangle_lines(draw_x, draw_y, BUTTON_WIDTH, BUTTON_HEIGHT, 2.0, BLUE);
    }

    let (mouse_x, mouse_y) = mouse_position();
    mouse_x >= draw_x && mouse_x <= draw_end_x && mouse_y >= draw_y && mouse_y <= draw_end_y
}
