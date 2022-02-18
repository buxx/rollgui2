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
            let dest_x = INVENTORY_BOX_MARGIN;
            let dest_y = INVENTORY_BOX_MARGIN;
            let width = screen_width() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;
            let height = screen_height() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;
            gui::inventory::draw_back(&self.graphics, dest_x, dest_y, width, height);

            let (mouse_x, mouse_y) = mouse_position();
            let mouse_is_hover_box = mouse_x > dest_x
                && mouse_x < dest_x + width as f32
                && mouse_y > dest_y
                && mouse_y < dest_y + height as f32;

            let columns = (width / gui::inventory::BUTTON_WIDTH) as usize;
            for (i, stuff) in inventory.stuff.iter().enumerate() {
                let row_i = i / columns;
                let col_i = i % columns;

                let stuff_dest_x = dest_x
                    + ((gui::inventory::BUTTON_WIDTH as f32 + gui::inventory::BUTTON_MARGIN)
                        * col_i as f32);
                let stuff_dest_y = dest_y
                    + ((gui::inventory::BUTTON_HEIGHT as f32 + gui::inventory::BUTTON_MARGIN)
                        * row_i as f32);

                let tile_id = self.graphics.find_tile_id_from_classes(&stuff.classes);
                gui::inventory::draw_item(&self.graphics, &tile_id, stuff_dest_x, stuff_dest_y);
            }

            if util::mouse_clicked() {
                if !mouse_is_hover_box {
                    self.inventory = None;
                }
            }
        }
    }
}
