use std::collections::HashMap;

use macroquad::prelude::*;

use crate::entity;

pub mod grid;
pub mod helper;

pub const BIG_BUTTON_SIZE: (f32, f32) = (96.0, 96.0);

pub struct UiDescriptionState {
    pub input_text_values: HashMap<String, String>,
}

impl Default for UiDescriptionState {
    fn default() -> Self {
        Self {
            input_text_values: HashMap::new(),
        }
    }
}

pub struct UiDescription {
    description: entity::description::Description,
    is_first_frame: bool,
    pub loading: bool,
}

pub enum UiDescriptionEvent {
    CloseDescription,
    FollowUrl(String),
}

impl UiDescription {
    pub fn new(description: entity::description::Description) -> Self {
        Self {
            description,
            is_first_frame: true,
            loading: false,
        }
    }

    pub fn draw(
        &mut self,
        egui_ctx: &egui::CtxRef,
        ui: &mut egui::Ui,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        self.check_init(egui_ctx, ui);
        let mut ui_message = None;

        if self.loading {
            ui.label("Loading...");
        }

        if self.description.is_grid {
            if let Some(ui_message_) = self.draw_grid(ui, state) {
                ui_message = Some(ui_message_);
            }
        } else {
            if let Some(ui_message_) = self.draw_default(ui, state) {
                ui_message = Some(ui_message_);
            }
        }

        if ui.button("Quitter").clicked() {
            ui_message = Some(UiDescriptionEvent::CloseDescription);
        }

        ui_message
    }

    pub fn draw_default(
        &self,
        ui: &mut egui::Ui,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        let mut event = None;

        for (i, part) in self.description.items.iter().enumerate() {
            match self.draw_part(ui, part, state) {
                Some(event_) => event = Some(event_),
                None => {}
            }
        }

        event
    }

    pub fn draw_part(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        let mut event = None;

        if part.is_link() {
            match self.draw_button(ui, part, state) {
                Some(event_) => event = Some(event_),
                None => {}
            }
        } else if part.is_text() {
            ui.label(part.label());
        } else if part.is_input() {
            self.draw_input(ui, part, state);
        } else if part.is_form {
            for form_part in &part.items {
                match self.draw_part(ui, form_part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
            }
        }

        event
    }
}
