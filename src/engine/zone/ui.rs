use macroquad::prelude::*;

use crate::{description, message};

impl super::ZoneEngine {
    pub fn ui(&mut self) -> Vec<message::MainMessage> {
        let mut messages = vec![];

        egui_macroquad::ui(|egui_ctx| {
            if let (Some(description), Some(description_state)) = (
                self.current_description.as_mut(),
                self.current_description_state.as_mut(),
            ) {
                let screen_width = screen_width();
                let screen_height = screen_height();
                let draw_to_x = 50.;
                let draw_to_y = 50.;
                let mut ui_message = None;

                egui::Window::new(&description.title())
                    .resizable(false)
                    .default_pos((draw_to_x, draw_to_y))
                    .default_size((screen_width - 50., screen_height - 50.))
                    .show(egui_ctx, |ui| {
                        ui_message = description.draw(egui_ctx, ui, description_state);
                    });

                if let Some(ui_message_) = ui_message {
                    match ui_message_ {
                        description::UiDescriptionEvent::CloseDescription => {
                            self.current_description = None;
                            self.current_description_state = None;
                        }
                        description::UiDescriptionEvent::FollowUrl(url) => {
                            self.description_request =
                                Some(self.client.get_description_request(url));
                            description.loading = true;
                        }
                        description::UiDescriptionEvent::FatalError(error) => {
                            messages.push(message::MainMessage::SetErrorEngine(error));
                        }
                        description::UiDescriptionEvent::ValidateFormInQuery(url) => {
                            let data = description_state.collect_form_data();
                            self.description_request =
                                Some(self.client.get_description_request_with_query(url, data));
                            description.loading = true;
                        }
                        description::UiDescriptionEvent::ValidateFormInBody(url) => {
                            let data = description_state.collect_form_data();
                            self.description_request =
                                Some(self.client.get_description_request_with_data(url, data));
                            description.loading = true;
                        }
                    }
                }
            }
        });

        messages
    }
}
