use super::{gui, log, ZoneEngine};
use macroquad::prelude::*;

use crate::util as base_util;

const AVATAR_DRAW_X: f32 = 40.;
const AVATAR_DRAW_Y: f32 = 30.;
const AVATAR_DRAW_WIDTH: f32 = 92.;
const AVATAR_DRAW_HEIGHT: f32 = 102.;

impl ZoneEngine {
    pub fn draw_left_panel(&mut self) {
        let highlight_button = self.get_highlighted_left_panel_button();

        gui::panel::draw_panel_background(&self.graphics);
        if let Some(button) = gui::panel::draw_buttons(
            &self.graphics,
            &self.current_left_panel_button,
            highlight_button,
        ) {
            self.disable_all_user_input = true;
            if base_util::mouse_clicked()
                && self.current_description.is_none()
                && self.inventory.is_none()
            {
                match &button.action(&self.state) {
                    gui::panel::ButtonAction::OpenDescription(url) => {
                        self.description_request =
                            Some(self.client.get_description_request(url.clone(), None, None));
                    }
                    gui::panel::ButtonAction::OpenInventory => {
                        self.make_open_inventory_request();
                    }
                }
                self.current_left_panel_button = Some(button.clone());
            }

            // Special case for inventory item dragging : reopen inventory if dragged on inventory button
            match &button {
                gui::panel::Button::Inventory => {
                    if let Some(inventory_state) = self.inventory_state.as_mut() {
                        if inventory_state.dragging_resource_i.is_some()
                            || inventory_state.dragging_stuff_i.is_some() && inventory_state.hide
                        {
                            // open it
                            inventory_state.hide = false;
                            inventory_state.must_hover_before_hide = true;
                        }
                    }
                }
                _ => {}
            }
        }

        self.draw_player_avatar();
    }

    pub fn draw_resume_items(&mut self) {
        if let Some(resume) = &self.resume {
            gui::resume::draw_resume_items(&self.graphics, resume);
        }
    }

    fn get_highlighted_left_panel_button(&self) -> Option<gui::panel::Button> {
        // If dragging an inventory item for dropping it, highlight the inventory button
        if let Some(inventory_state) = &self.inventory_state {
            if (inventory_state.dragging_resource_i.is_some()
                || inventory_state.dragging_stuff_i.is_some())
                && inventory_state.hide
            {
                return Some(gui::panel::Button::Inventory);
            }
        }

        None
    }

    pub fn draw_player_avatar(&self) {
        let avatar_uuid = self.state.player.private_avatar_uuid();
        if let Some(avatar_texture) = self.graphics.avatars.get(&avatar_uuid) {
            draw_texture_ex(
                *avatar_texture,
                AVATAR_DRAW_X,
                AVATAR_DRAW_Y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(AVATAR_DRAW_WIDTH, AVATAR_DRAW_HEIGHT)),
                    ..Default::default()
                },
            );
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
