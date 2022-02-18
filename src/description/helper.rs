use macroquad::prelude::*;

use super::UiDescription;
use crate::entity;

impl UiDescription {
    pub fn title(&self) -> String {
        match &self.description.title {
            Some(title) => title.clone(),
            None => "__NO_TITLE__".to_string(),
        }
    }

    pub fn check_init(&mut self, egui_ctx: &egui::CtxRef, ui: &mut egui::Ui) {
        if self.is_first_frame {
            let mut style = (*egui_ctx.style()).clone();
            // TODO : with new egui, do https://discord.com/channels/900275882684477440/900275883124858921/938081008568377354
            style.override_text_style = Some(egui::TextStyle::Heading);
            egui_ctx.set_style(style);
        }
        self.is_first_frame = false;
    }

    pub fn draw_button(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        let label = part.label();
        if ui.button(&label).clicked() {
            if let Some(url) = &part.form_action {
                event = Some(super::UiDescriptionEvent::FollowUrl(url.clone()));
            } else {
                error!("Description button '{}' has no form action", &label);
            }
        };

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
                ui.add(egui::TextEdit::singleline(value).hint_text(part.label()));
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
                // FIXME BS NOW : au POST il faut remettre le suffix
                let value = state
                    .numeric_values
                    .entry(name.to_string())
                    .or_insert(default_value);
                let mut input = egui::DragValue::new(value).speed(1.0);
                if let Some(suffix_) = suffix {
                    input = input.suffix(suffix_);
                }
                ui.add(input).on_hover_text(part.label());
            }
        }

        event
    }
}
