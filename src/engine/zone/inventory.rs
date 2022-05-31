use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entity, util};

use super::gui;

const INVENTORY_BOX_MARGIN: f32 = 50.;

fn was_scroll(last_begin_click_coordinates: Option<Vec2>) -> bool {
    if let Some(last_position) = last_begin_click_coordinates {
        let mouse_position = Vec2::from(mouse_position());
        let change_vector = mouse_position - last_position;
        if change_vector.x > 3.
            || change_vector.y > 3.
            || change_vector.x < -3.
            || change_vector.y < -3.
        {
            return true;
        }
    }

    false
}

fn scroll_value(last_begin_click_coordinates: Vec2) -> Vec2 {
    let mouse_position = Vec2::from(mouse_position());
    mouse_position - last_begin_click_coordinates
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    stuff: Vec<entity::stuff::StuffApi>,
    resource: Vec<entity::resource::ResourceApi>,
    weight: f32,
    clutter: f32,
    over_weight: bool,
    over_clutter: bool,
}

pub struct InventoryState {
    pub active_stuff_i: Option<usize>,
    pub dragging_stuff_i: Option<usize>,
    pub active_resource_i: Option<usize>,
    pub dragging_resource_i: Option<usize>,
    pub help_text: Option<String>,
    pub drop_request: Option<quad_net::http_request::Request>,
    pub hide: bool,
    pub must_hover_before_hide: bool,
    pub scroll_value: f32,
    pub last_scroll_value: f32,
    pub begin_click_in_inventory: Option<bool>,
}

impl Default for InventoryState {
    fn default() -> Self {
        Self {
            active_stuff_i: None,
            dragging_stuff_i: None,
            active_resource_i: None,
            dragging_resource_i: None,
            help_text: None,
            drop_request: None,
            hide: false,
            must_hover_before_hide: false,
            scroll_value: 0.0,
            last_scroll_value: 0.0,
            begin_click_in_inventory: None,
        }
    }
}

impl super::ZoneEngine {
    pub fn proceed_inventory_requests(&mut self) {
        self.proceed_inventory_request();
        self.proceed_inventory_drop_request();
    }

    fn proceed_inventory_request(&mut self) {
        if let Some(request) = self.inventory_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(inventory_string) => {
                        let inventory: Inventory = serde_json::from_str(&inventory_string).unwrap();
                        self.setup_inventory(inventory);
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

    fn proceed_inventory_drop_request(&mut self) {
        if let Some(request) = self.inventory_drop_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(description_string) => {
                        match entity::description::Description::from_string(&description_string) {
                            Ok(description) => {
                                if let Some(message) = description.quick_action_response {
                                    let message_level = if description.is_quick_error {
                                        super::log::UserLogLevel::Error
                                    } else {
                                        super::log::UserLogLevel::Info
                                    };
                                    self.user_logs
                                        .push(super::log::UserLog::new(message, message_level));
                                }
                                self.make_open_inventory_request();
                            }
                            Err(error) => {
                                error!("Error while decoding drop request description : {}", error);
                            }
                        };
                    }
                    Err(error) => {
                        error!("Error while requiring drop request description : {}", error);
                    }
                }
                self.current_left_panel_button = None;
                self.description_request = None;
            }
        }
    }

    fn setup_inventory(&mut self, inventory: Inventory) {
        self.inventory = Some(inventory);
        self.inventory_state = Some(InventoryState::default());
    }

    pub fn make_open_inventory_request(&mut self) {
        self.inventory_request = Some(self.client.get_inventory_request(&self.state.player.id));
    }

    pub fn draw_inventory(&mut self) {
        if let (Some(inventory), Some(inventory_state)) =
            (&self.inventory, self.inventory_state.as_mut())
        {
            // Default help text
            inventory_state.help_text = None;
            if inventory.over_weight || inventory.over_clutter {
                let help_text = if inventory.over_weight && inventory.over_clutter {
                    "Surcharge ! (poid et encombrement)"
                } else if inventory.over_weight {
                    "Surcharge ! (poid)"
                } else {
                    "Surcharge ! (encombrement)"
                };
                inventory_state.help_text = Some(help_text.to_string());
            }
        }

        if let (Some(inventory), Some(inventory_state)) =
            (&self.inventory, self.inventory_state.as_mut())
        {
            let mut mouse_is_hover_stuff: Option<usize> = None;
            let mut mouse_is_hover_resource: Option<usize> = None;
            let mut mouse_is_hover_box = false;

            if !inventory_state.hide {
                let box_dest_x = INVENTORY_BOX_MARGIN;
                let box_dest_y = INVENTORY_BOX_MARGIN;
                let box_width = screen_width() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;
                let box_height = screen_height() - INVENTORY_BOX_MARGIN - INVENTORY_BOX_MARGIN;

                mouse_is_hover_box = gui::inventory::draw_back(
                    &self.graphics,
                    box_dest_x,
                    box_dest_y,
                    box_width,
                    box_height,
                );

                if mouse_is_hover_box {
                    inventory_state.must_hover_before_hide = false;
                }

                let columns = (box_width
                    / (gui::inventory::BUTTON_WIDTH + gui::inventory::BUTTON_MARGIN))
                    as usize;
                // let max_rows = ((box_height / 2.)
                //     / (gui::inventory::BUTTON_HEIGHT + gui::inventory::BUTTON_MARGIN))
                //     as usize;

                // if max_rows < 1 {
                //     self.helper_text = Some("Écran trop petit pout inventaire !".to_string());
                // }

                // if max_rows > 0 {
                let start_draw_stuff_x = box_dest_x + (gui::inventory::BUTTON_MARGIN / 2.);
                let start_draw_stuff_y = box_dest_y
                    + (gui::inventory::BUTTON_MARGIN / 2.)
                    + inventory_state.scroll_value.min(0.);
                let mut last_draw_y = box_dest_y;
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

                    // let drawing_last_available_row = row_i == max_rows - 1;
                    let drawing_last_column = col_i == columns - 1;
                    let drawing_last_stuff = i == inventory.stuff.len() - 1;

                    let stuff_quantity = if stuff.count > 1 {
                        Some(stuff.count.to_string())
                    } else {
                        None
                    };

                    // // If all available rows done and there is more than this stuff, don't draw this stuff
                    // if drawing_last_available_row && drawing_last_column && !drawing_last_stuff {
                    //     gui::inventory::draw_more(&self.graphics, stuff_dest_x, stuff_dest_y);
                    //     break;
                    // } else {
                    let (stuff_dest_x, stuff_dest_y) =
                        if let Some(dragged_stuff_i) = inventory_state.dragging_stuff_i {
                            if dragged_stuff_i == i {
                                let mouse_position = mouse_position();
                                (mouse_position.0, mouse_position.1)
                            } else {
                                (stuff_dest_x, stuff_dest_y)
                            }
                        } else {
                            (stuff_dest_x, stuff_dest_y)
                        };

                    let tile_id = self.graphics.find_tile_id_from_classes(&stuff.classes);
                    let force_border = if let Some(active_stuff_i) = inventory_state.active_stuff_i
                    {
                        active_stuff_i == i
                    } else {
                        false
                    };
                    if gui::inventory::draw_item(
                        &self.graphics,
                        &tile_id,
                        stuff_dest_x,
                        stuff_dest_y,
                        stuff_quantity,
                        stuff.is_cumbersome || stuff.is_heavy,
                        stuff.is_equip,
                        force_border,
                    ) {
                        mouse_is_hover_stuff = Some(i);
                        inventory_state.help_text = Some(stuff.infos.clone());
                    }
                    // }
                }

                let start_draw_resource_x = box_dest_x + (gui::inventory::BUTTON_MARGIN / 2.);
                let start_draw_resource_y = last_draw_y
                    + gui::inventory::BUTTON_HEIGHT as f32
                    + gui::inventory::BUTTON_MARGIN;
                for (i, resource) in inventory.resource.iter().enumerate() {
                    let row_i = i / columns;
                    let col_i = i % columns;
                    let resource_dest_x = start_draw_resource_x
                        + ((gui::inventory::BUTTON_WIDTH as f32 + gui::inventory::BUTTON_MARGIN)
                            * col_i as f32);
                    let resource_dest_y = start_draw_resource_y
                        + ((gui::inventory::BUTTON_HEIGHT as f32 + gui::inventory::BUTTON_MARGIN)
                            * row_i as f32);

                    // let drawing_last_available_row = row_i == max_rows - 1;
                    let drawing_last_column = col_i == columns - 1;
                    let drawing_last_resource = i == inventory.resource.len() - 1;

                    // // If all available rows done and there is more than this resource, don't draw this resource
                    // if drawing_last_available_row && drawing_last_column && !drawing_last_resource {
                    //     gui::inventory::draw_more(&self.graphics, resource_dest_x, resource_dest_y);
                    //     break;
                    // } else {
                    let (resource_dest_x, resource_dest_y) =
                        if let Some(dragged_resource_i) = inventory_state.dragging_resource_i {
                            if dragged_resource_i == i {
                                let mouse_position = mouse_position();
                                (mouse_position.0, mouse_position.1)
                            } else {
                                (resource_dest_x, resource_dest_y)
                            }
                        } else {
                            (resource_dest_x, resource_dest_y)
                        };

                    let tile_id = self.graphics.find_tile_id_from_classes(&resource.classes);
                    let force_border =
                        if let Some(active_resource_i) = inventory_state.active_resource_i {
                            active_resource_i == i
                        } else {
                            false
                        };
                    if gui::inventory::draw_item(
                        &self.graphics,
                        &tile_id,
                        resource_dest_x,
                        resource_dest_y,
                        None,
                        resource.is_cumbersome || resource.is_heavy,
                        false,
                        force_border,
                    ) {
                        mouse_is_hover_resource = Some(i);
                        inventory_state.help_text = Some(resource.infos.clone());
                    }
                    // }
                }
                // }

                if let Some(help_text) = &inventory_state.help_text {
                    draw_text(
                        help_text,
                        box_dest_x,
                        box_dest_y + box_height + gui::inventory::HELP_TEXT_HEIGHT - 5.0,
                        gui::inventory::HELP_TEXT_HEIGHT,
                        BLACK,
                    );
                }
            } else {
                // Draw selected stuff/resource under the cursor
                let mouse_position = Vec2::from(mouse_position());

                if let Some(dragged_stuff_i) = inventory_state.dragging_stuff_i {
                    let stuff = &inventory.stuff[dragged_stuff_i];
                    let tile_id = self.graphics.find_tile_id_from_classes(&stuff.classes);
                    draw_texture_ex(
                        self.graphics.tileset_texture,
                        mouse_position.x,
                        mouse_position.y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(
                                self.graphics
                                    .tiles_mapping
                                    .get(&tile_id)
                                    .unwrap()
                                    .to_rect(0),
                            ),
                            ..Default::default()
                        },
                    )
                } else if let Some(dragged_resource_i) = inventory_state.dragging_resource_i {
                    let resource = &inventory.resource[dragged_resource_i];
                    let tile_id = self.graphics.find_tile_id_from_classes(&resource.classes);
                    draw_texture_ex(
                        self.graphics.tileset_texture,
                        mouse_position.x,
                        mouse_position.y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(
                                self.graphics
                                    .tiles_mapping
                                    .get(&tile_id)
                                    .unwrap()
                                    .to_rect(0),
                            ),
                            ..Default::default()
                        },
                    )
                }

                // When inventory is hide, it means prepare to drop, so highlight the tile under the cursor
                self.highlight_tiles
                    .push((self.mouse_zone_coordinates.0, self.mouse_zone_coordinates.1));
            }

            if util::mouse_clicked() {
                if inventory_state.dragging_resource_i.is_none()
                    && inventory_state.dragging_stuff_i.is_none()
                {
                    if let Some(mouse_is_hover_stuff) = mouse_is_hover_stuff {
                        if let Some(active_stuff_i) = inventory_state.active_stuff_i {
                            if active_stuff_i == mouse_is_hover_stuff {
                                let stuff_id: i32 = inventory.stuff[mouse_is_hover_stuff]
                                    .ids
                                    .first()
                                    .unwrap()
                                    .clone();
                                let request = self
                                    .client
                                    .get_look_at_inventory_stuff(&self.state.player.id, stuff_id);
                                self.description_request = Some(request);
                                self.current_left_panel_button =
                                    Some(gui::panel::Button::Inventory);
                            } else {
                                println!(
                                    "Select inventory stuff at position '{}'",
                                    mouse_is_hover_stuff
                                );
                                inventory_state.active_resource_i = None;
                                inventory_state.active_stuff_i = Some(mouse_is_hover_stuff)
                            }
                        } else {
                            println!(
                                "Select inventory stuff at position '{}'",
                                mouse_is_hover_stuff
                            );
                            inventory_state.active_resource_i = None;
                            inventory_state.active_stuff_i = Some(mouse_is_hover_stuff)
                        }
                    } else if let Some(mouse_is_hover_resource) = mouse_is_hover_resource {
                        if let Some(active_resource_i) = inventory_state.active_resource_i {
                            if active_resource_i == mouse_is_hover_resource {
                                let resource_id: String =
                                    inventory.resource[mouse_is_hover_resource].id.clone();
                                let request = self.client.get_look_at_inventory_resource(
                                    &self.state.player.id,
                                    &resource_id,
                                );
                                self.description_request = Some(request);
                                self.current_left_panel_button =
                                    Some(gui::panel::Button::Inventory);
                            } else {
                                println!(
                                    "Select inventory resource at position '{}'",
                                    mouse_is_hover_resource
                                );
                                inventory_state.active_stuff_i = None;
                                inventory_state.active_resource_i = Some(mouse_is_hover_resource)
                            }
                        } else {
                            println!(
                                "Select inventory resource at position '{}'",
                                mouse_is_hover_resource
                            );
                            inventory_state.active_stuff_i = None;
                            inventory_state.active_resource_i = Some(mouse_is_hover_resource)
                        }
                    } else {
                        inventory_state.active_resource_i = None;
                        inventory_state.active_stuff_i = None;
                    }
                }

                if !mouse_is_hover_box {
                    if inventory_state.dragging_stuff_i.is_some()
                        || inventory_state.dragging_resource_i.is_some()
                    {
                        let post_base_url = if let Some(dragged_stuff_i) =
                            inventory_state.dragging_stuff_i
                        {
                            let stuff = &inventory.stuff[dragged_stuff_i];
                            stuff.drop_base_url.clone()
                        } else if let Some(dragged_resource_i) = inventory_state.dragging_resource_i
                        {
                            let resource = &inventory.resource[dragged_resource_i];
                            resource.drop_base_url.clone()
                        } else {
                            panic!("Should not be here")
                        };
                        self.pending_events
                            .push(super::UserEvent::InventoryItemDropped(
                                self.mouse_zone_coordinates.0,
                                self.mouse_zone_coordinates.1,
                                post_base_url,
                            ));
                    }

                    // Close inventory only if not scrolling
                    if !was_scroll(self.last_begin_click_coordinates_this_frame) {
                        self.inventory = None;
                    }
                }

                inventory_state.dragging_stuff_i = None;
                inventory_state.dragging_resource_i = None;
                inventory_state.begin_click_in_inventory = None;
                inventory_state.last_scroll_value = inventory_state.scroll_value;
            } else if util::mouse_pressed() {
                if inventory_state.begin_click_in_inventory.is_none() {
                    inventory_state.begin_click_in_inventory = Some(mouse_is_hover_box);
                }
                if inventory_state.dragging_stuff_i.is_none()
                    && inventory_state.dragging_resource_i.is_none()
                {
                    if was_scroll(self.last_begin_click_coordinates) {
                        let change_vector =
                            scroll_value(self.last_begin_click_coordinates.unwrap());
                        // Move from stuff icon
                        let dragging_resource_or_stuff =
                            if let (Some(mouse_is_hover_stuff), Some(active_stuff_i)) =
                                (mouse_is_hover_stuff, inventory_state.active_stuff_i)
                            {
                                if active_stuff_i == mouse_is_hover_stuff {
                                    inventory_state.dragging_stuff_i = Some(mouse_is_hover_stuff);
                                    true
                                } else {
                                    false
                                }

                            // Move from resource icon
                            } else if let (Some(mouse_is_hover_resource), Some(active_resource_i)) =
                                (mouse_is_hover_resource, inventory_state.active_resource_i)
                            {
                                if active_resource_i == mouse_is_hover_resource {
                                    inventory_state.dragging_resource_i =
                                        Some(mouse_is_hover_resource);
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            };

                        if !dragging_resource_or_stuff {
                            inventory_state.scroll_value =
                                change_vector.y + inventory_state.last_scroll_value;
                        }
                    }
                } else {
                    if !mouse_is_hover_box {
                        if !inventory_state.must_hover_before_hide {
                            inventory_state.hide = true;
                        }
                    }
                }
            }
        }
    }

    pub fn inventory_item_dropped(
        &mut self,
        zone_row_i: usize,
        zone_col_i: usize,
        post_base_url: String,
    ) {
        // Check if not hover a tile
        if let Some(_tile_id) = self.state.map.tile_id(zone_row_i, zone_col_i) {
            let post_url = format!(
                "{}&zone_row_i={}&zone_col_i={}&quick_action=1",
                post_base_url, zone_row_i, zone_col_i
            );

            // Do the drop request
            self.current_left_panel_button = Some(gui::panel::Button::Inventory);
            self.inventory_drop_request =
                Some(self.client.get_description_request(post_url, None, None));
        }
    }
}
