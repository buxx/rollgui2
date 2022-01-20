use crate::engine::root::state;

use macroquad::prelude::*;

pub fn scene(state: &state::RootState) {
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    draw_text(&state.hello_text, 20.0, 20.0, 30.0, DARKGRAY);
}
