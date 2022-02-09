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

    let concrete_mouse_x = mouse_zone_position.x * state.map.concrete_width as f32;
    let concrete_mouse_y = mouse_zone_position.y * state.map.concrete_height as f32;

    let mouse_hover = concrete_mouse_x >= dest_x
        && concrete_mouse_x <= dest_x + graphics.tile_width
        && concrete_mouse_y <= dest_y
        && concrete_mouse_y >= dest_y - graphics.tile_height;

    let tile_id = if mouse_hover {
        "TILE_HOVER"
    } else {
        "TILE_SELECTION"
    };

    graphics.draw_tile_in_camera(
        map.concrete_width,
        map.concrete_height,
        dest_x,
        dest_y,
        tile_id,
        None,
        tick_i,
        None,
        None,
    );

    // Draw the exploitable tile class
    let exploitable_tile_id = graphics.find_tile_id_from_classes(&exploitable_tile.classes);
    graphics.draw_tile_in_camera(
        map.concrete_width,
        map.concrete_height,
        dest_x,
        dest_y,
        &exploitable_tile_id,
        None,
        0,
        None,
        None,
    );

    mouse_hover
}
