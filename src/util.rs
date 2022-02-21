use macroquad::prelude::*;

pub fn convert_to_local(pixel_pos: Vec2) -> Vec2 {
    Vec2::new(pixel_pos.x / screen_width(), pixel_pos.y / screen_height()) * 2.0
        - Vec2::new(1.0, 1.0)
}

pub fn mouse_clicked() -> bool {
    is_mouse_button_released(MouseButton::Left)
}

pub fn mouse_pressed() -> bool {
    is_mouse_button_down(MouseButton::Left)
}

// TODO : use to_digit (failed when tried ...)
pub fn char_to_key_code(char_: &char) -> Option<KeyCode> {
    match char_ {
        'P' => Some(KeyCode::P),
        _ => None,
    }
}
