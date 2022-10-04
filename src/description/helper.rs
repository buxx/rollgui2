use macroquad::prelude::*;

use super::{UiDescription, UiDescriptionEvent};
use crate::{entity, ui::utils::egui_scale};

pub const BIG_BUTTON_SIZE: [f32; 2] = [150.0, 150.0];
pub const IMG_BUTTON_SIZE: [f32; 2] = [64., 64.];

impl UiDescription {
    pub fn title(&self) -> String {
        match &self.description.title {
            Some(title) => title.clone(),
            None => "__NO_TITLE__".to_string(),
        }
    }

    pub fn check_init(
        &mut self,
        egui_ctx: &egui::Context,
        ui: &mut egui::Ui,
    ) -> Option<UiDescriptionEvent> {
        if self.is_first_frame {
            let mut style = (*egui_ctx.style()).clone();
            // FIXME BS NOW : still required ?
            // TODO : with new egui, do https://discord.com/channels/900275882684477440/900275883124858921/938081008568377354
            style.override_text_style = Some(egui::TextStyle::Heading);
            egui_ctx.set_style(style);

            // Illustration case
            if let Some(illustration_name) = &self.description.illustration_name {
                if let Some(illustration_data) = self.graphics.illustrations.get(illustration_name)
                {
                    let illustration_texture: egui::TextureHandle = ui
                        .ctx()
                        .load_texture(illustration_name, illustration_data.clone());
                    self.illustration_texture = Some(illustration_texture);
                } else if !self.illustration_load_requested {
                    self.illustration_load_requested = true;
                    return Some(UiDescriptionEvent::RequireIllustrationLoad(
                        illustration_name.clone(),
                    ));
                }
            }

            // Prepare textures for tiles images
            for part in &self.description.items {
                let classes = if part.use_classes2_for_button {
                    &part.classes2
                } else {
                    &part.classes
                };
                let tile_id = &self.graphics.find_tile_id_from_classes(classes);
                if tile_id != "UNKNOWN" {
                    if let Some(image_data) = self.graphics.tiles_data.get(tile_id) {
                        let texture: egui::TextureHandle =
                            ui.ctx().load_texture(tile_id, image_data.clone());
                        self.tiles_textures.insert(tile_id.to_string(), texture);

                        // Prepare texture with action points if needed
                        if let Some(cost) = part.cost {
                            if let Some(tile_with_ap) = self.graphics.tile_with_ap(tile_id, cost) {
                                let name = format!("{}__{}AP", tile_id, cost);
                                let texture: egui::TextureHandle =
                                    ui.ctx().load_texture(&name, tile_with_ap.clone());
                                self.tiles_textures.insert(name, texture);
                            }
                        }
                    }
                }
            }
            self.is_first_frame = false;
        }
        None
    }

    pub fn draw_button(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        _state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        let label = part.label();
        let classes = if part.use_classes2_for_button {
            &part.classes2
        } else {
            &part.classes
        };
        let tile_id = self.graphics.find_tile_id_from_classes(classes);

        let clicked = if self.draw_big_button {
            ui.add_sized(BIG_BUTTON_SIZE, egui::Button::new(&label))
                .clicked()
        } else {
            let tile_name = if let Some(cost) = part.cost {
                if cost > 0. {
                    format!("{}__{}AP", &tile_id, cost)
                } else {
                    tile_id.clone()
                }
            } else {
                tile_id.clone()
            };

            if let Some(texture) = self.tiles_textures.get(&tile_name) {
                ui.add(egui::ImageButton::new(
                    texture,
                    egui::Vec2::new(IMG_BUTTON_SIZE[0], IMG_BUTTON_SIZE[1]),
                ))
                .on_hover_text(&label)
                .clicked()
            } else {
                ui.add(egui::Button::new(&label)).clicked()
            }
        };

        if clicked {
            if let Some(url) = &part.form_action {
                event = Some(super::UiDescriptionEvent::FollowUrl(url.clone()));
            } else {
                error!("Description button '{}' has no form action", &label);
            }
        };

        event
    }

    pub fn draw_buttons_group(
        &self,
        ui: &mut egui::Ui,
        parts: &Vec<entity::description::Part>,
        state: &mut super::UiDescriptionState,
        link_group_name: &str,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        for part in parts {
            if let Some(link_group_name_) = &part.link_group_name {
                if link_group_name_ == link_group_name {
                    if let Some(event_) = self.draw_button(ui, part, state) {
                        event = Some(event_);
                    }
                }
            }
        }

        event
    }

    pub fn draw_input(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        let name = match &part.name {
            Some(name) => name,
            None => {
                return Some(super::UiDescriptionEvent::FatalError(format!(
                    "Missing name for input"
                )));
            }
        };
        let type_ = match &part.type_ {
            Some(type_) => match type_.as_str() {
                "NUMBER" => entity::description::InputType::Numeric,
                "STRING" => entity::description::InputType::String,
                _ => {
                    return Some(super::UiDescriptionEvent::FatalError(format!(
                        "Unknown input type '{}'",
                        type_
                    )));
                }
            },
            None => {
                return Some(super::UiDescriptionEvent::FatalError(format!(
                    "Missing type for input '{}'",
                    &name
                )));
            }
        };

        match type_ {
            entity::description::InputType::String => {
                let default_value = match &part.default_value {
                    Some(default_value) => default_value,
                    None => "",
                };
                let value = state
                    .string_values
                    .entry(name.to_string())
                    .or_insert(default_value.to_string());
                ui.label(part.label());
                let is_password = part.classes.contains(&"password".to_string());
                if ui
                    .add(egui::TextEdit::singleline(value).password(is_password))
                    .gained_focus()
                {
                    event = Some(super::UiDescriptionEvent::TextEditFocused(
                        part.label().to_string(),
                        name.to_string(),
                        value.clone(),
                    ));
                }
            }
            entity::description::InputType::Numeric => {
                let (default_value, suffix) = match part.analyze_default_value() {
                    Some((default_value, suffix)) => {
                        if let Some(suffix_) = suffix {
                            (
                                default_value
                                    .replace(&suffix_, "")
                                    .trim()
                                    .parse::<f32>()
                                    .unwrap(),
                                Some(suffix_),
                            )
                        } else {
                            (default_value.parse::<f32>().unwrap(), None)
                        }
                    }
                    None => (0.0, None),
                };

                let (value, _) = state
                    .numeric_values
                    .entry(name.to_string())
                    .or_insert((default_value, suffix.clone()));

                ui.label(part.label());

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().slider_width = ui.available_width() / egui_scale();

                    if let (Some(min_value), Some(max_value)) = (part.min_value, part.max_value) {
                        *value = value.min(max_value);
                        *value = value.max(min_value);

                        let mut widget = egui::Slider::new(value, min_value..=max_value);
                        if let Some(suffix_) = suffix {
                            widget = widget.suffix(suffix_);
                        }
                        ui.add(widget);
                    } else {
                        let mut widget = egui::DragValue::new(value).speed(1.0);
                        if let Some(suffix_) = suffix {
                            widget = widget.suffix(suffix_);
                        }
                        ui.add(widget);
                    };
                });

                if !ui.ctx().is_using_pointer() {
                    if part.expect_integer {
                        *value = value.round()
                    }
                }
            }
        }

        event
    }

    pub fn draw_checkbox(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let event = None;

        let name = match &part.name {
            Some(name) => name,
            None => {
                return Some(super::UiDescriptionEvent::FatalError(format!(
                    "Missing name for checkbox"
                )));
            }
        };
        let checked = state
            .boolean_values
            .entry(name.to_string())
            .or_insert(part.checked);

        ui.label("");
        ui.add(egui::Checkbox::new(checked, &part.label()));

        event
    }

    pub fn draw_choices(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let event = None;

        if let Some(choices) = &part.choices {
            let name = match &part.name {
                Some(name) => name,
                None => {
                    return Some(super::UiDescriptionEvent::FatalError(format!(
                        "Missing name for choice"
                    )));
                }
            };
            let default_value = match &part.value {
                Some(default_value) => default_value,
                None => "",
            };
            let value = state
                .string_values
                .entry(name.to_string())
                .or_insert(default_value.to_string());

            let mut new_value = None;
            ui.horizontal(|ui| {
                for choice in choices.iter() {
                    if ui
                        .add(egui::RadioButton::new(value == choice, choice))
                        .clicked()
                    {
                        new_value = Some(choice.clone());
                    }
                }
            });

            if let Some(new_value) = new_value {
                state.string_values.insert(name.clone(), new_value);
            }
        }

        event
    }
}
