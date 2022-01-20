use crate::engine::root::state;

pub enum RootUiEvent {
    QuitGame,
    ChangeHelloText(String),
    OpenZone,
}

pub fn ui(_state: &state::RootState) -> Option<RootUiEvent> {
    let mut ui_message = None;

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
            if ui.button("Hello").clicked() {
                ui_message = Some(RootUiEvent::ChangeHelloText("42 !!".to_string()));
            }
            if ui.button("Zone").clicked() {
                ui_message = Some(RootUiEvent::OpenZone);
            }
            if ui.button("Quitter").clicked() {
                ui_message = Some(RootUiEvent::QuitGame);
            }
        });
    });

    ui_message
}
