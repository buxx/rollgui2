use super::{gui, ZoneEngine, LEFT_PANEL_WIDTH, QUICK_ACTION_MARGIN};
use crate::{action as base_action, ui::utils::is_mobile, util as base_util};
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
        let start_draw_x = LEFT_PANEL_WIDTH + QUICK_ACTION_MARGIN;
        let start_draw_y = screen_height()
            - (gui::quick::BUTTON_HEIGHT * button_size_factor())
            - QUICK_ACTION_MARGIN;
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

            let pressed_by_key = if let Some(quick_action_key_) = &quick_action.quick_action_key {
                is_key_pressed(
                    base_util::char_to_key_code(quick_action_key_).expect("Update key mapping !"),
                )
            } else {
                false
            };

            if hover || pressed_by_key {
                self.helper_text = Some(quick_action.name.clone());
                if base_util::mouse_clicked() || pressed_by_key {
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
                self.disable_all_user_input = true;
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
