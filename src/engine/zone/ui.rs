use macroquad::prelude::*;

use crate::{description, message, ui::utils::egui_scale, util};

impl super::ZoneEngine {
    pub fn ui(&mut self) -> Vec<message::MainMessage> {
        let mut messages = vec![];

        egui_macroquad::ui(|egui_ctx| {
            if let (Some(description), Some(description_state)) = (
                self.current_description.as_mut(),
                self.current_description_state.as_mut(),
            ) {
                let mut ui_message = None;

                let _response = egui::Window::new(&description.title())
                    .resizable(false)
                    .default_pos((0., 0.))
                    .fixed_size((
                        (screen_width() / egui_scale()) - 12.0,
                        (screen_height() / egui_scale()) - 35.0,
                    ))
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
                                Some(self.client.get_description_request(url, None, None));
                            description.loading = true;
                        }
                        description::UiDescriptionEvent::FatalError(error) => {
                            messages.push(message::MainMessage::SetErrorEngine(error));
                        }
                        description::UiDescriptionEvent::ValidateFormInQuery(url) => {
                            let data = description_state.collect_form_data();
                            self.description_request =
                                Some(self.client.get_description_request(url, Some(data), None));
                            description.loading = true;
                        }
                        description::UiDescriptionEvent::ValidateFormInBody(url) => {
                            let data = description_state.collect_form_data();
                            self.description_request =
                                Some(self.client.get_description_request(url, None, Some(data)));
                            description.loading = true;
                        }
                        description::UiDescriptionEvent::SetDescriptionUi(mut new_description) => {
                            new_description.loading = false;
                            self.current_description = Some(*new_description);
                            self.current_description_state =
                                Some(description::UiDescriptionState::default());
                        }
                        // Managed inside of UiDescription
                        description::UiDescriptionEvent::TextEditFocused(_, _, _) => {}
                        description::UiDescriptionEvent::RequireIllustrationLoad(
                            illustration_name,
                        ) => {
                            messages
                                .push(message::MainMessage::LoadIllustration(illustration_name));
                        }
                    }
                }

                // To know later if its a dragging from egui, note if starting click is in egui
                if util::mouse_pressed() && self.last_begin_click_was_in_egui.is_none() {
                    self.last_begin_click_was_in_egui = Some(egui_ctx.is_pointer_over_area());
                }

                if util::mouse_clicked() {
                    if let Some(last_begin_click_was_in_egui_) = self.last_begin_click_was_in_egui {
                        // Close egui only if the begin and end of click was outside egui
                        if !last_begin_click_was_in_egui_ && !egui_ctx.is_pointer_over_area() {
                            self.current_description = None;
                            self.current_description_state = None;
                        }
                    }
                    self.last_begin_click_was_in_egui = None;
                }
            }
        });

        messages
    }
}
