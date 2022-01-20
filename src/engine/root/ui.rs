use crate::event;

pub fn ui() -> Option<event::UiEvent> {
    let mut ui_message = None;

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
            if ui.button("Hello").clicked() {
                ui_message = Some(event::UiEvent::QuitGame);
            }
            if ui.button("Quitter").clicked() {
                ui_message = Some(event::UiEvent::QuitGame);
            }
        });
    });

    ui_message
}
