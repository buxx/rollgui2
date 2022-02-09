use crate::{action, engine::zone::state, graphics};
use macroquad::prelude::*;

pub fn draw_action_tile_in_camera(
    graphics: &graphics::Graphics,
    state: &state::ZoneState,
    exploitable_tile: &action::ExploitableTile,
    tick_i: i16,
    mouse_zone_position: Vec2,
) -> bool {
    let map = &state.map;

    let dest_x = exploitable_tile.zone_col_i as f32 * graphics.tile_width;
    let dest_y = exploitable_tile.zone_row_i as f32 * graphics.tile_height;

    let concrete_mouse_x = mouse_zone_position.x * state.map.width as f32;
    let concrete_mouse_y = mouse_zone_position.y * state.map.height as f32;

    info!(
        "{}.{} vs {}.{}",
        dest_x, dest_y, concrete_mouse_x, concrete_mouse_y
    );

    let mouse_hover = concrete_mouse_x >= dest_x
        && concrete_mouse_x <= dest_x + graphics.tile_width
        && concrete_mouse_y >= dest_y
        && concrete_mouse_y <= dest_y + graphics.tile_height;

    let tick_i_ = if mouse_hover { 0 } else { tick_i };

    graphics.draw_tile_in_camera(
        map.concrete_width,
        map.concrete_height,
        dest_x,
        dest_y,
        "TILE_SELECTION",
        None,
        tick_i_,
        None,
        None,
    );

    mouse_hover
}
