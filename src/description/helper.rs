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

        let name = part.name.clone().unwrap();
        let value = state
            .input_text_values
            .entry(name)
            .or_insert("".to_string());
        ui.label(part.label());
        ui.add(egui::TextEdit::singleline(value).hint_text("Write something here"));

        event
    }
}
