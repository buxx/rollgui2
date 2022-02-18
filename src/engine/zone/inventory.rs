use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entity, util};

use super::gui;

const INVENTORY_BOX_MARGIN: f32 = 150.;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    stuff: Vec<entity::stuff::StuffApi>,
    resource: Vec<entity::resource::ResourceApi>,
    weight: f32,
    clutter: f32,
}

impl super::ZoneEngine {
    pub fn proceed_inventory_requests(&mut self) {
        if let Some(request) = self.inventory_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(inventory_string) => {
                        let inventory: Inventory = serde_json::from_str(&inventory_string).unwrap();
                        self.inventory = Some(inventory);
                    }
                    Err(error) => {
                        error!("Error while requiring inventory : {}", error);
                    }
                }
                self.current_left_panel_button = None;
                self.inventory_request = None;
            }
        }
    }

    pub fn draw_inventory(&mut self) {
        if let Some(inventory) = &self.inventory {
            let box_dest_x = INVENTORY_BOX_MARGIN;
            let box_dest_y = INVENTORY_BOX_MARGIN;
            let box_width = screen_width() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;
            let box_height = screen_height() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;

            let mouse_is_hover_box = gui::inventory::draw_back(
                &self.graphics,
                box_dest_x,
                box_dest_y,
                box_width,
                box_height,
            );
            let mut mouse_is_hover_stuff: Option<usize> = None;
            let mut mouse_is_hover_resource: Option<usize> = None;

            let columns = (box_width
                / (gui::inventory::BUTTON_WIDTH + gui::inventory::BUTTON_MARGIN))
                as usize;
            let max_rows = ((box_height / 2.)
                / (gui::inventory::BUTTON_HEIGHT + gui::inventory::BUTTON_MARGIN))
                as usize;

            if max_rows < 1 {
                self.helper_text = Some("Écran trop petit pout inventaire !".to_string());
                return;
            }

            let start_draw_stuff_x = box_dest_x + (gui::inventory::BUTTON_MARGIN / 2.);
            let start_draw_stuff_y = box_dest_y + (gui::inventory::BUTTON_MARGIN / 2.);
            let mut last_draw_y = 0.;
            for (i, stuff) in inventory.stuff.iter().enumerate() {
                let row_i = i / columns;
                let col_i = i % columns;
                let stuff_dest_x = start_draw_stuff_x
                    + ((gui::inventory::BUTTON_WIDTH as f32 + gui::inventory::BUTTON_MARGIN)
                        * col_i as f32);
                let stuff_dest_y = start_draw_stuff_y
                    + ((gui::inventory::BUTTON_HEIGHT as f32 + gui::inventory::BUTTON_MARGIN)
                        * row_i as f32);
                last_draw_y = stuff_dest_y;

                let drawing_last_available_row = row_i == max_rows - 1;
                let drawing_last_column = col_i == columns - 1;
                let drawing_last_stuff = i == inventory.stuff.len() - 1;

                // If all available rows done and there is more than this stuff, don't draw this stuff
                if drawing_last_available_row && drawing_last_column && !drawing_last_stuff {
                    gui::inventory::draw_more(&self.graphics, stuff_dest_x, stuff_dest_y);
                    break;
                } else {
                    let tile_id = self.graphics.find_tile_id_from_classes(&stuff.classes);
                    if gui::inventory::draw_item(
                        &self.graphics,
                        &tile_id,
                        stuff_dest_x,
                        stuff_dest_y,
                    ) {
                        mouse_is_hover_stuff = Some(i);
                    }
                }
            }

            let start_draw_resource_x = box_dest_x;
            let start_draw_resource_y =
                last_draw_y + gui::inventory::BUTTON_HEIGHT as f32 + gui::inventory::BUTTON_MARGIN;
            for (i, resource) in inventory.resource.iter().enumerate() {
                let row_i = i / columns;
                let col_i = i % columns;
                let resource_dest_x = start_draw_resource_x
                    + ((gui::inventory::BUTTON_WIDTH as f32 + gui::inventory::BUTTON_MARGIN)
                        * col_i as f32);
                let resource_dest_y = start_draw_resource_y
                    + ((gui::inventory::BUTTON_HEIGHT as f32 + gui::inventory::BUTTON_MARGIN)
                        * row_i as f32);

                let drawing_last_available_row = row_i == max_rows - 1;
                let drawing_last_column = col_i == columns - 1;
                let drawing_last_resource = i == inventory.resource.len() - 1;

                // If all available rows done and there is more than this resource, don't draw this resource
                if drawing_last_available_row && drawing_last_column && !drawing_last_resource {
                    gui::inventory::draw_more(&self.graphics, resource_dest_x, resource_dest_y);
                    break;
                } else {
                    let tile_id = self.graphics.find_tile_id_from_classes(&resource.classes);
                    if gui::inventory::draw_item(
                        &self.graphics,
                        &tile_id,
                        resource_dest_x,
                        resource_dest_y,
                    ) {
                        mouse_is_hover_resource = Some(i);
                    }
                }
            }

            if util::mouse_clicked() {
                if let Some(mouse_is_hover_stuff) = mouse_is_hover_stuff {
                    let stuff_id: i32 = inventory.stuff[mouse_is_hover_stuff]
                        .ids
                        .first()
                        .unwrap()
                        .clone();
                    let request = self
                        .client
                        .get_look_at_inventory_stuff(&self.state.player.id, stuff_id);
                    self.description_request = Some(request);
                    self.current_left_panel_button = Some(gui::panel::Button::Inventory);
                } else if let Some(mouse_is_hover_resource) = mouse_is_hover_resource {
                    let resource_id: String =
                        inventory.resource[mouse_is_hover_resource].id.clone();
                    let request = self
                        .client
                        .get_look_at_inventory_resource(&self.state.player.id, &resource_id);
                    self.description_request = Some(request);
                    self.current_left_panel_button = Some(gui::panel::Button::Inventory);
                }

                if !mouse_is_hover_box {
                    self.inventory = None;
                }
            }
        }
    }
}
