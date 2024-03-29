use super::{
    gui::{self, chat::display::Display as ChatDisplay},
    ZoneEngine, LEFT_PANEL_WIDTH, QUICK_ACTION_MARGIN,
};
use crate::{
    action as base_action,
    ui::utils::{egui_scale, is_mobile},
    util::{self as base_util, mouse_clicked, mouse_pressed},
};
use macroquad::prelude::*;

pub fn button_size_factor() -> f32 {
    if is_mobile() {
        2.0
    } else {
        1.0
    }
}

impl ZoneEngine {
    pub fn draw_quick_actions(&mut self, action_clicked: bool) {
        let mut start_draw_x =
            LEFT_PANEL_WIDTH + QUICK_ACTION_MARGIN + self.quick_action_x_offset.unwrap_or(0.);

        // If there is a dragging, update the start_draw_x
        let mouse_pos = mouse_position();
        if let Some(click_begin_in_quick_action) = self.click_begin_in_quick_action {
            if click_begin_in_quick_action != mouse_pos {
                start_draw_x = start_draw_x + (mouse_pos.0 - click_begin_in_quick_action.0)
            }
        }

        let bottom_offset = if self.chat_state.is_display() {
            let chat_display = ChatDisplay::from_env();
            match chat_display {
                ChatDisplay::Bottom => chat_display.height() * egui_scale(),
                _ => 0.,
            }
        } else {
            0.
        };
        let start_draw_y = screen_height()
            - (gui::quick::BUTTON_HEIGHT * button_size_factor())
            - QUICK_ACTION_MARGIN
            - bottom_offset;
        let mut quick_action_just_clicked = false;

        for (i, quick_action) in self.quick_actions.iter().enumerate() {
            let decal = i as f32
                * ((gui::quick::BUTTON_WIDTH * button_size_factor()) + QUICK_ACTION_MARGIN);
            let draw_x = start_draw_x + decal;
            let draw_y = start_draw_y;
            // TODO : optimize ?
            let tile_id1 = self
                .graphics
                .find_tile_id_from_classes(&quick_action.classes1);
            let tile_id2 = self
                .graphics
                .find_tile_id_from_classes(&quick_action.classes2);

            let active = if let Some(selected_quick_action) = self.selected_quick_action {
                selected_quick_action == i
            } else {
                false
            };

            let hover = gui::quick::draw_quick_action_button(
                &self.graphics,
                active,
                &tile_id1,
                &tile_id2,
                draw_x,
                draw_y,
                &quick_action.quick_action_key,
                self.tick_i,
            );

            let pressed_by_key = quick_action.quick_action_key_pressed();
            if !self.chat_state.is_input_focused() && (hover || pressed_by_key) {
                self.helper_text = Some(quick_action.name.clone());
                let direct_click = self.click_begin_in_quick_action.unwrap_or((0., 0.))
                    == mouse_pos
                    && mouse_clicked();
                if direct_click || pressed_by_key {
                    self.click_begin_in_quick_action = None;
                    if quick_action.force_open_description {
                        self.description_request = Some(self.client.get_description_request(
                            quick_action.base_url.clone(),
                            None,
                            None,
                        ));
                    } else {
                        if !quick_action.direct_action {
                            self.current_action =
                                Some(base_action::Action::from_quick_action(&quick_action));
                            self.selected_quick_action = Some(i);
                            self.pending_exploitable_tiles = vec![];
                            quick_action_just_clicked = true;
                        } else {
                            self.quick_action_requests
                                .push(self.client.get_quick_action_request(
                                    &quick_action.uuid,
                                    &quick_action.base_url,
                                    None,
                                    None,
                                ));
                        }
                    }
                }
                self.disable_all_user_input = true;
            }

            // Click just begin hover a quick action
            if hover && mouse_pressed() && self.click_begin_in_quick_action.is_none() {
                debug!("Click begin hover a quick action");
                self.click_begin_in_quick_action = Some(mouse_position());
            }

            // Dragging just finished
            if let Some(click_begin_in_quick_action) = self.click_begin_in_quick_action {
                // Dragging imply mouse position changed
                if mouse_clicked() && click_begin_in_quick_action != mouse_pos {
                    let offset = (mouse_pos.0 - click_begin_in_quick_action.0)
                        + self.quick_action_x_offset.unwrap_or(0.);
                    debug!("Quick action drag just ended (offset: {})", offset);
                    self.quick_action_x_offset = Some(offset);
                    self.click_begin_in_quick_action = None;
                }
            }
        }

        if base_util::mouse_clicked()
            && !quick_action_just_clicked
            && !action_clicked
            && self.quick_action_requests.len() == 0
        {
            self.selected_quick_action = None;
            self.current_action = None;
            self.pending_exploitable_tiles = vec![];
        }
    }

    pub fn draw_current_action(&mut self) -> bool {
        let mut exploitable_tile_clicked: Option<usize> = None;

        if let Some(current_action) = &self.current_action {
            for (i, exploitable_tile) in current_action.exploitable_tiles.iter().enumerate() {
                let tile_is_pending = self.pending_exploitable_tiles.contains(&i);
                let hover = gui::action::draw_action_tile_in_camera(
                    &self.graphics,
                    &self.state,
                    exploitable_tile,
                    self.tick_i,
                    self.mouse_zone_position,
                    tile_is_pending,
                );

                let pressed_by_key = if let Some(associated_key_) = &current_action.associated_key {
                    is_key_pressed(*associated_key_)
                } else {
                    false
                };
                if hover || pressed_by_key {
                    // If exploitable tile clicked or keyboard key pressed
                    if (base_util::mouse_clicked() || pressed_by_key)
                    // But exploitable tile is not currently pending
                        && !self.pending_exploitable_tiles.contains(&i)
                    {
                        exploitable_tile_clicked = Some(i);
                        self.quick_action_requests
                            .push(self.client.get_quick_action_request(
                                &current_action.uuid,
                                &current_action.post_url,
                                Some(exploitable_tile.zone_row_i),
                                Some(exploitable_tile.zone_col_i),
                            ));
                    } else {
                        self.helper_text = Some(exploitable_tile.infos.clone());
                    }
                }
            }

            if let Some(exploitable_tile_clicked_) = exploitable_tile_clicked {
                if current_action.all_tiles_at_once {
                    self.pending_exploitable_tiles =
                        (0..current_action.exploitable_tiles.len()).collect();
                } else {
                    self.pending_exploitable_tiles = vec![exploitable_tile_clicked_];
                }
            }
        }

        exploitable_tile_clicked.is_some()
    }
}
