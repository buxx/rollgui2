use crate::{engine::zone::state, entity};

pub enum ZoneUiEvent {
    ReturnToRoot,
}

pub fn ui(
    _state: &state::ZoneState,
    description: &Option<entity::description::Description>,
) -> Option<ZoneUiEvent> {
    let mut ui_message = None;

    egui_macroquad::ui(|egui_ctx| {
        if let Some(description) = description {
            let title = description.title();
            egui::Window::new(title).show(egui_ctx, |ui| {
                if ui.button("Quitter").clicked() {
                    ui_message = Some(ZoneUiEvent::ReturnToRoot);
                }
            });
        }
    });

    ui_message
}
