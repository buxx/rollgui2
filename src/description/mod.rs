use std::collections::HashMap;

use macroquad::prelude::*;

use crate::entity;
use crate::ui as base_ui;
use crate::ui::utils::is_mobile;

pub mod grid;
pub mod helper;

pub const BIG_BUTTON_SIZE: (f32, f32) = (96.0, 96.0);

#[derive(Clone)]
pub struct UiDescriptionState {
    pub string_values: HashMap<String, String>,
    pub numeric_values: HashMap<String, (f32, Option<String>)>, // field_name, (value, suffix)
    pub boolean_values: HashMap<String, bool>,
    pub error_message: Option<String>,
}

impl Default for UiDescriptionState {
    fn default() -> Self {
        Self {
            string_values: HashMap::new(),
            numeric_values: HashMap::new(),
            boolean_values: HashMap::new(),
            error_message: None,
        }
    }
}

#[derive(Clone)]
pub struct UiDescription {
    pub description: entity::description::Description,
    pub previous: Option<Box<UiDescription>>,
    pub is_first_frame: bool,
    pub loading: bool,
    pub draw_big_button: bool,
    pub text_input_request: Option<base_ui::text_input::TextInputRequest>,
}

pub enum UiDescriptionEvent {
    CloseDescription,
    FollowUrl(String),
    FatalError(String),
    ValidateFormInQuery(String),
    ValidateFormInBody(String),
    SetDescriptionUi(Box<UiDescription>),
    TextEditFocused(String, String, String), // title, name, value
}

impl UiDescription {
    pub fn new(
        description: entity::description::Description,
        previous: Option<UiDescription>,
    ) -> Self {
        let previous = if let Some(previous_) = previous {
            Some(Box::new(previous_))
        } else {
            None
        };
        Self {
            description,
            previous,
            is_first_frame: true,
            loading: false,
            draw_big_button: false,
            text_input_request: None,
        }
    }

    fn manage_pending(&mut self, state: &mut UiDescriptionState) {
        if let Some(request) = &mut self.text_input_request {
            if let Some(new_value) = request.try_recv() {
                let key = request.name().to_string();
                state.string_values.insert(key, new_value);
                self.text_input_request = None;
            }
        }
    }

    pub fn draw(
        &mut self,
        egui_ctx: &egui::Context,
        ui: &mut egui::Ui,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        self.check_init(egui_ctx, ui);
        self.manage_pending(state);

        let mut ui_message = None;

        ui.horizontal(|ui| {
            if ui.button("Fermer").clicked() {
                ui_message = Some(UiDescriptionEvent::CloseDescription);
            };
            if let Some(previous_) = &self.previous {
                if ui.button("Précédent").clicked() {
                    ui_message = Some(UiDescriptionEvent::SetDescriptionUi(previous_.clone()));
                }
            };
        });

        ui.separator();

        if self.loading {
            ui.label("Loading...");
            return ui_message;
        }

        if let Some(error_message) = &state.error_message {
            ui.colored_label(egui::Color32::RED, error_message);
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

        // Manage some messages
        match &ui_message {
            Some(UiDescriptionEvent::TextEditFocused(title, name, value)) => {
                if is_mobile() {
                    self.text_input_request = Some(base_ui::text_input::TextInputRequest::new(
                        title.to_string(),
                        name.to_string(),
                        value.to_string(),
                    ));
                }
            }
            _ => {}
        }

        ui_message
    }

    pub fn draw_default(
        &self,
        ui: &mut egui::Ui,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        let mut event = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("root_grid")
                .num_columns(2)
                // .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (_i, part) in self.description.items.iter().enumerate() {
                        match self.draw_part(ui, part, state) {
                            Some(event_) => event = Some(event_),
                            None => {}
                        }
                        ui.end_row();
                    }
                });
        });

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
            if let (Some(label), Some(text)) = (&part.label, &part.text) {
                ui.add(egui::Label::new(format!("{} :", label)));
                ui.add(egui::Label::new(text).wrap(true));
            } else if let Some(label) = &part.label {
                ui.add(egui::Label::new(label));
            } else if let Some(text) = &part.text {
                ui.add(egui::Label::new(format!("")));
                ui.add(egui::Label::new(text).wrap(true));
            }
        } else if part.is_input() {
            match self.draw_input(ui, part, state) {
                Some(event_) => event = Some(event_),
                None => {}
            }
        } else if part.is_checkbox() {
            match self.draw_checkbox(ui, part, state) {
                Some(event_) => event = Some(event_),
                None => {}
            }
        } else if part.is_form {
            for form_part in &part.items {
                match self.draw_part(ui, form_part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
                ui.end_row();
            }
            if ui.button("Valider").clicked() {
                if let Some(url) = &part.form_action {
                    if part.form_values_in_query {
                        event = Some(UiDescriptionEvent::ValidateFormInQuery(url.clone()));
                    } else {
                        event = Some(UiDescriptionEvent::ValidateFormInBody(url.clone()));
                    }
                } else {
                    error!("Description form has no form action");
                }
            };
        }

        event
    }
}

impl UiDescriptionState {
    pub fn collect_form_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut data = serde_json::Map::new();

        for (key, value) in &self.string_values {
            data.insert(key.clone(), serde_json::json!(value));
        }

        for (key, (value, suffix)) in &self.numeric_values {
            if let Some(suffix_) = suffix {
                data.insert(
                    key.clone(),
                    serde_json::json!(format!("{} {}", value, suffix_)),
                );
            } else {
                data.insert(key.clone(), serde_json::json!(value));
            }
        }

        for (key, value) in &self.boolean_values {
            if *value {
                data.insert(key.clone(), serde_json::json!("on"));
            }
        }

        data
    }
}
