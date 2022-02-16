use macroquad::prelude::*;

use crate::{engine::zone::state, entity};

pub enum ZoneUiEvent {
    ReturnToRoot,
}

pub fn ui(
    _state: &state::ZoneState,
    description: &Option<entity::description::Description>,
) -> Option<ZoneUiEvent> {
    let mut ui_message = None;

    let screen_width = screen_width();
    let screen_height = screen_height();
    let draw_to_x = 50.;
    let draw_to_y = 50.;

    egui_macroquad::ui(|egui_ctx| {
        if let Some(description) = description {
            let title = description.title();
            egui::Window::new(title)
                .resizable(false)
                .default_pos((draw_to_x, draw_to_y))
                .default_size((screen_width - 50., screen_height - 50.))
                .show(egui_ctx, |ui| {
                    if ui.button("Quitter").clicked() {
                        ui_message = Some(ZoneUiEvent::ReturnToRoot);
                    }
                });
        }
    });

    ui_message
}
