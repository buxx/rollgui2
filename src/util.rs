use macroquad::prelude::*;

pub fn convert_to_local(pixel_pos: Vec2) -> Vec2 {
    Vec2::new(pixel_pos.x / screen_width(), pixel_pos.y / screen_height()) * 2.0
        - Vec2::new(1.0, 1.0)
}

pub fn mouse_clicked() -> bool {
    is_mouse_button_released(MouseButton::Left)
}
