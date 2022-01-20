use crate::engine::zone::state;

pub enum ZoneUiEvent {
    ReturnToRoot,
}

pub fn ui(state: &state::ZoneState) -> Option<ZoneUiEvent> {
    let mut ui_message = None;

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Zone").show(egui_ctx, |ui| {
            if ui.button("Quitter").clicked() {
                ui_message = Some(ZoneUiEvent::ReturnToRoot);
            }
        });
    });

    ui_message
}
