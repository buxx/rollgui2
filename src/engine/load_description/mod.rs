use crate::{client, description, message};

pub struct LoadDescriptionEngine {
    pub request: quad_net::http_request::Request,
    pub client: Option<client::Client>,
    pub previous_ui_description: Option<description::UiDescription>,
    pub previous_ui_description_state: Option<description::UiDescriptionState>,
}

impl LoadDescriptionEngine {
    pub fn new(
        request: quad_net::http_request::Request,
        client: Option<client::Client>,
        previous_ui_description: Option<description::UiDescription>,
        previous_ui_description_state: Option<description::UiDescriptionState>,
    ) -> Self {
        Self {
            request,
            client,
            previous_ui_description,
            previous_ui_description_state,
        }
    }
}

impl super::Engine for LoadDescriptionEngine {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        // Description Request
        if let Some(request_result) = self.request.try_recv() {
            match client::Client::description_from_request_data(request_result) {
                Ok(description) => {
                    return vec![message::MainMessage::SetDescriptionEngine(
                        description,
                        self.client.clone(),
                    )]
                }
                Err(error_message) => {
                    // Set previous description with error message if any previous description
                    if let (Some(ui_description), Some(ui_description_state)) = (
                        &self.previous_ui_description,
                        &self.previous_ui_description_state,
                    ) {
                        let ui_description_ = ui_description.clone();
                        let mut ui_description_state_ = ui_description_state.clone();
                        ui_description_state_.error_message = Some(error_message);

                        return vec![message::MainMessage::SetDescriptionEngineFrom(
                            ui_description_,
                            ui_description_state_,
                            self.client.clone(),
                        )];
                    }
                    return vec![message::MainMessage::SetErrorEngine(format!(
                        "Erreur : {}",
                        error_message
                    ))];
                }
            }
        }

        // UI
        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                ui.colored_label(egui::Color32::LIGHT_GRAY, "Chargement ...");
            });
        });
        egui_macroquad::draw();

        vec![]
    }

    fn signal_illustration_loaded(&mut self, _illustration_name: &str) {}

    fn replace_graphics(&mut self, _graphics: crate::graphics::Graphics) {}
}
