use crate::gui;

use super::{ZoneEngine, LEFT_PANEL_WIDTH, QUICK_ACTION_MARGIN};
use crate::{action as base_action, util as base_util};
use macroquad::prelude::*;

impl ZoneEngine {
    pub fn draw_quick_actions(&mut self, action_clicked: bool) {
        let start_draw_x = LEFT_PANEL_WIDTH as f32 + QUICK_ACTION_MARGIN;
        let start_draw_y = screen_height() - gui::quick::BUTTON_HEIGHT - QUICK_ACTION_MARGIN;
        let mut quick_action_just_clicked = false;

        for (i, quick_action) in self.quick_actions.iter().enumerate() {
            let decal = i as f32 * (gui::quick::BUTTON_WIDTH + QUICK_ACTION_MARGIN);
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

            if gui::quick::draw_quick_action_button(
                &self.graphics,
                active,
                &tile_id1,
                &tile_id2,
                draw_x,
                draw_y,
                self.tick_i,
            ) {
                if base_util::mouse_clicked() {
                    self.current_action =
                        Some(base_action::Action::from_quick_action(&quick_action));
                    self.selected_quick_action = Some(i);
                    self.pending_exploitable_tiles = vec![];
                    quick_action_just_clicked = true;
                }
                self.disable_all_user_input = true;
            }
        }

        if base_util::mouse_clicked() && !quick_action_just_clicked && !action_clicked {
            self.selected_quick_action = None;
            self.current_action = None;
            self.pending_exploitable_tiles = vec![];
        }
    }

    pub fn draw_current_action(&mut self) -> bool {
        let mut exploitable_tile_clicked: Option<usize> = None;

        if let Some(current_action) = &self.current_action {
            for (i, exploitable_tile) in current_action.exploitable_tiles.iter().enumerate() {
                let exploitable_tile_blinking = self.pending_exploitable_tiles.contains(&i);
                if gui::action::draw_action_tile_in_camera(
                    &self.graphics,
                    &self.state,
                    exploitable_tile,
                    self.tick_i,
                    self.mouse_zone_position,
                    exploitable_tile_blinking,
                ) {
                    if base_util::mouse_clicked() {
                        exploitable_tile_clicked = Some(i);
                        self.quick_action_requests
                            .push(self.client.get_quick_action_request(
                                &current_action.uuid,
                                &current_action.post_url,
                                exploitable_tile.zone_row_i,
                                exploitable_tile.zone_col_i,
                            ));
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
