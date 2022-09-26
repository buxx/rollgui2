use macroquad::prelude::*;

use crate::{engine::zone::web_socket, util::mouse_clicked};

use super::{util::click_action_event, ZoneEngine, LEFT_PANEL_WIDTH};

impl ZoneEngine {
    fn draw_pending_request_click(&self) {
        if let Some((request_clicks, row_i, col_i)) = &self.pending_request_clicks {
            if self.tick_i % 2 == 0 {
                let tile_id = self
                    .graphics
                    .find_tile_id_from_classes(&request_clicks.cursor_classes);
                self.graphics.draw_tile_in_camera(
                    self.state.map.concrete_width,
                    self.state.map.concrete_height,
                    *col_i as f32 * self.graphics.tile_height,
                    *row_i as f32 * self.graphics.tile_width,
                    &tile_id,
                    None,
                    self.tick_i,
                    None,
                    None,
                );
            }
        }
    }

    fn draw_current_request_click(&mut self) {
        // This draw happen only if there is click request
        if self.request_clicks.is_none() {
            return;
        }
        let request_clicks = self.request_clicks.as_ref().expect("Must exist here");

        // Ensure mouse not hover left panel
        if (mouse_position().0 <= LEFT_PANEL_WIDTH && mouse_clicked())
            || is_key_pressed(KeyCode::Escape)
            || is_mouse_button_released(MouseButton::Right)
        {
            self.request_clicks = None;

            return;
        }

        let tile_id = self
            .graphics
            .find_tile_id_from_classes(&request_clicks.cursor_classes);
        let concrete_mouse_x = self.mouse_zone_position.x * self.state.map.concrete_width as f32;
        let concrete_mouse_y = self.mouse_zone_position.y * self.state.map.concrete_height as f32;
        let tile_hovered_row_i = (concrete_mouse_y / self.graphics.tile_width) as i32 + 1;
        let tile_hovered_col_i = (concrete_mouse_x / self.graphics.tile_height) as i32;

        // Draw cursor tile
        self.graphics.draw_tile_in_camera(
            self.state.map.concrete_width,
            self.state.map.concrete_height,
            tile_hovered_col_i as f32 * self.graphics.tile_width,
            tile_hovered_row_i as f32 * self.graphics.tile_height,
            &tile_id,
            None,
            self.tick_i,
            None,
            None,
        );

        // Is there a click
        if mouse_clicked() {
            let request_clicks = request_clicks.clone();

            // If it was a single click, remove request clicks from state
            if !request_clicks.many {
                self.request_clicks = None;
            }

            // Make the request
            debug!(
                "Request click on {}.{} for {}/{}",
                tile_hovered_row_i,
                tile_hovered_col_i,
                &request_clicks.action_type,
                request_clicks.action_description_id
            );
            web_socket(&self.state).send_text(&click_action_event(
                &request_clicks,
                tile_hovered_row_i as i16,
                tile_hovered_col_i as i16,
            ));
            self.pending_request_clicks =
                Some((request_clicks, tile_hovered_row_i, tile_hovered_col_i));
        }
    }

    pub fn draw_request_clicks(&mut self) {
        self.draw_current_request_click();
        self.draw_pending_request_click();
    }
}
