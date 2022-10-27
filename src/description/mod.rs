use std::collections::HashMap;

use macroquad::prelude::*;

use crate::engine::zone::util::tight_display;
use crate::entity;
use crate::graphics;
use crate::ui as base_ui;
use crate::ui::utils::egui_scale;
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
    pub already_displayed_groups: Vec<String>,
}

impl Default for UiDescriptionState {
    fn default() -> Self {
        Self {
            string_values: HashMap::new(),
            numeric_values: HashMap::new(),
            boolean_values: HashMap::new(),
            error_message: None,
            already_displayed_groups: vec![],
        }
    }
}

#[derive(Clone)]
pub struct UiDescription {
    pub description: entity::description::Description,
    pub graphics: graphics::Graphics,
    pub previous: Option<Box<UiDescription>>,
    pub is_first_frame: bool,
    pub loading: bool,
    pub draw_big_button: bool,
    pub text_input_request: Option<base_ui::text_input::TextInputRequest>,
    pub tiles_textures: HashMap<String, egui::TextureHandle>,
    pub illustration_texture: Option<egui::TextureHandle>,
    pub illustration_load_requested: bool,
}

pub enum UiDescriptionEvent {
    CloseDescription,
    FollowUrl(String),
    FatalError(String),
    ValidateFormInQuery(String),
    ValidateFormInBody(String),
    SetDescriptionUi(Box<UiDescription>),
    TextEditFocused(String, String, String), // title, name, value
    RequireIllustrationLoad(String),
}

impl UiDescription {
    pub fn new(
        description: entity::description::Description,
        graphics: graphics::Graphics,
        previous: Option<UiDescription>,
    ) -> Self {
        let previous = if let Some(previous_) = previous {
            Some(Box::new(previous_))
        } else {
            None
        };
        Self {
            description,
            graphics,
            previous,
            is_first_frame: true,
            loading: false,
            draw_big_button: false,
            text_input_request: None,
            tiles_textures: HashMap::new(),
            illustration_texture: None,
            illustration_load_requested: false,
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
        let mut ui_message = None;

        if let Some(event) = self.check_init(egui_ctx, ui) {
            ui_message = Some(event);
        };
        self.manage_pending(state);

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
            if is_mobile() {
                ui.vertical(|ui| {
                    if let Some(ui_message_) = self.draw_default(ui, state) {
                        ui_message = Some(ui_message_);
                    }

                    // Curiously, end page is sometime cropped
                    ui.label(" ");
                    ui.separator();
                });
            } else {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Some(ui_message_) = self.draw_default(ui, state) {
                            ui_message = Some(ui_message_);
                        }

                        // Curiously, end page is sometime cropped
                        ui.label(" ");
                        ui.separator();
                    });
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
        state.already_displayed_groups = vec![];
        let mut event = None;

        if let Some(illustration_texture) = &self.illustration_texture {
            ui.horizontal_top(|ui| {
                let width = (screen_width().min(768.) / egui_scale()) * 0.95;
                let height = 300.0 / (768.0 / width);
                ui.image(illustration_texture, egui::Vec2::new(width, height));
            });
            ui.separator();
        }

        for part in &self.description.items {
            // Group
            if let Some(link_group_name) = &part.link_group_name {
                if !state.already_displayed_groups.contains(link_group_name) {
                    ui.add(egui::Label::new(
                        egui::RichText::new(link_group_name).heading(),
                    ));
                    ui.horizontal_wrapped(|ui| {
                        if let Some(event_) = self.draw_buttons_group(
                            ui,
                            &self.description.items,
                            state,
                            link_group_name,
                        ) {
                            event = Some(event_)
                        }
                    });
                    state.already_displayed_groups.push(link_group_name.clone());
                }
            // Non-Group
            } else {
                if let Some(event_) = self.draw_part(ui, part, state) {
                    event = Some(event_)
                }
            }
        }

        if self.description.footer_links.len() > 0 {
            ui.horizontal_top(|ui| {
                for footer_link in &self.description.footer_links {
                    match self.draw_part(ui, footer_link, state) {
                        Some(event_) => event = Some(event_),
                        None => {}
                    }
                }
            });
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

        // Column
        if part.columns != 0 {
            egui::Grid::new("a_grid")
                .num_columns(part.columns as usize)
                .striped(true)
                .min_col_width(screen_width() / 2.0)
                .show(ui, |ui| {
                    // determine how rows columns count
                    let mut max_row_count = 0;
                    for part_ in &part.items {
                        if part_.is_column {
                            if part_.items.len() > max_row_count {
                                max_row_count = part_.items.len();
                            }
                        }
                    }

                    // Draw row per row
                    for row_i in 0..max_row_count {
                        for part_ in &part.items {
                            if part_.is_column {
                                if let Some(part__) = part_.items.get(row_i) {
                                    if let Some(event_) = self.draw_part(ui, part__, state) {
                                        event = Some(event_)
                                    }
                                }
                            }
                        }
                        ui.end_row();
                    }
                });

        // Not-Column
        } else {
            if part.is_link() {
                match self.draw_button(ui, part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
            } else if part.is_text() {
                if let (Some(label), Some(text)) = (&part.label, &part.text) {
                    ui.add(egui::Label::new(format!("{} : {}", label, text)));
                } else if let Some(label) = &part.label {
                    ui.add(egui::Label::new(label));
                } else if let Some(text) = &part.text {
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
            } else if part.is_choices() {
                match self.draw_choices(ui, part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
            } else if part.is_form {
                if tight_display() {
                    ui.vertical(|ui| {
                        match self.draw_form(ui, part, state) {
                            Some(event_) => event = Some(event_),
                            None => {}
                        };
                    });
                } else {
                    egui::Grid::new("root_grid")
                        .num_columns(2)
                        .striped(true)
                        .min_col_width(screen_width() / 2.0 / egui_scale())
                        .show(ui, |ui| {
                            match self.draw_form(ui, part, state) {
                                Some(event_) => event = Some(event_),
                                None => {}
                            };
                        });
                };
            }
        }

        event
    }

    fn draw_form(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut UiDescriptionState,
    ) -> Option<UiDescriptionEvent> {
        let mut event = None;

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

        return event;
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
