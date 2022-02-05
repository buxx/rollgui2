use crate::message;

use super::Engine;

pub struct ErrorEngine {
    error_message: String,
}

impl ErrorEngine {
    pub fn new(error_message: String) -> Self {
        Self { error_message }
    }
}

impl Engine for ErrorEngine {
    fn run(&mut self) -> Vec<message::MainMessage> {
        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                ui.colored_label(egui::Color32::RED, &self.error_message);
            });
        });

        vec![]
    }
}
