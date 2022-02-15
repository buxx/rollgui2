use super::{gui, log, ZoneEngine};
use macroquad::prelude::*;

use crate::util as base_util;

impl ZoneEngine {
    pub fn draw_left_panel(&mut self) {
        gui::panel::draw_panel_background(&self.graphics);
        if let Some(button) =
            gui::panel::draw_buttons(&self.graphics, &self.current_left_panel_button)
        {
            self.disable_all_user_input = true;
            if base_util::mouse_clicked() {
                match button.action(&self.state) {
                    gui::panel::ButtonAction::OpenDescription(url) => {
                        self.description_request = Some(self.client.get_description_request(url));
                    }
                }
                self.current_left_panel_button = Some(button);
            }
        }
    }

    pub fn draw_helper_text(&self) {
        let draw_x = 10.;
        let draw_y = screen_height() - log::LOG_BOX_HEIGHT - (super::HELPER_TEXT_FONT_SIZE / 2.);
        draw_rectangle(
            draw_x,
            draw_y - super::HELPER_TEXT_FONT_SIZE,
            log::LOG_BOX_WIDTH,
            super::HELPER_TEXT_FONT_SIZE,
            GRAY,
        );
        if let Some(helper_text) = &self.helper_text {
            draw_text(
                helper_text,
                draw_x,
                draw_y - (super::HELPER_TEXT_FONT_SIZE / 4.),
                super::HELPER_TEXT_FONT_SIZE,
                BLACK,
            );
        }
    }
}
