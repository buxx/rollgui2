use macroquad::prelude::*;

use crate::entity;

pub mod grid;

pub const BIG_BUTTON_SIZE: (f32, f32) = (96.0, 96.0);

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

    pub(crate) fn title(&self) -> String {
        match &self.description.title {
            Some(title) => title.clone(),
            None => "__NO_TITLE__".to_string(),
        }
    }

    fn check_init(&mut self, egui_ctx: &egui::CtxRef, ui: &mut egui::Ui) {
        if self.is_first_frame {
            let mut style = (*egui_ctx.style()).clone();
            // TODO : with new egui, do https://discord.com/channels/900275882684477440/900275883124858921/938081008568377354
            style.override_text_style = Some(egui::TextStyle::Heading);
            egui_ctx.set_style(style);
        }
        self.is_first_frame = false;
    }

    pub fn draw(
        &mut self,
        egui_ctx: &egui::CtxRef,
        ui: &mut egui::Ui,
    ) -> Option<UiDescriptionEvent> {
        self.check_init(egui_ctx, ui);
        let mut ui_message = None;

        if self.loading {
            ui.label("Loading...");
        }

        if self.description.is_grid {
            if let Some(ui_message_) = self.draw_grid(ui) {
                ui_message = Some(ui_message_);
            }
        } else {
            if let Some(ui_message_) = self.draw_default(ui) {
                ui_message = Some(ui_message_);
            }
        }

        if ui.button("Quitter").clicked() {
            ui_message = Some(UiDescriptionEvent::CloseDescription);
        }

        ui_message
    }

    pub fn draw_default(&mut self, ui: &mut egui::Ui) -> Option<UiDescriptionEvent> {
        let mut event = None;

        for (i, part) in self.description.items.iter().enumerate() {
            if self.is_link(part) {
                match self.draw_button(ui, part) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
            }
        }

        event
    }

    fn is_link(&self, part: &entity::description::Part) -> bool {
        part.form_action.is_some() && !part.is_form
    }

    fn draw_button(
        &self,
        ui: &mut egui::Ui,
        part: &entity::description::Part,
    ) -> Option<UiDescriptionEvent> {
        let mut event = None;

        let label = part.label();
        if ui.button(&label).clicked() {
            if let Some(url) = &part.form_action {
                event = Some(UiDescriptionEvent::FollowUrl(url.clone()));
            } else {
                error!("Description button '{}' has no form action", &label);
            }
        };

        event
    }
}
