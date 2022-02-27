use crate::{description, entity, message};

pub struct DescriptionEngine {
    pub ui_description: description::UiDescription,
    pub ui_description_state: description::UiDescriptionState,
}

impl DescriptionEngine {
    pub fn new(description: entity::description::Description) -> Self {
        Self {
            ui_description: description::UiDescription::new(description, None),
            ui_description_state: description::UiDescriptionState {
                ..Default::default()
            },
        }
    }

    pub fn from_state(
        ui_description: description::UiDescription,
        ui_description_state: description::UiDescriptionState,
    ) -> Self {
        Self {
            ui_description,
            ui_description_state,
        }
    }
}

impl super::Engine for DescriptionEngine {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        let mut event = None;

        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                if let Some(error_message) = &self.ui_description.error_message {
                    ui.colored_label(egui::Color32::RED, error_message);
                }

                event = self
                    .ui_description
                    .draw(egui_ctx, ui, &mut self.ui_description_state);
            });
        });

        if let Some(event_) = event {
            match event_ {
                description::UiDescriptionEvent::FatalError(error) => {
                    return vec![message::MainMessage::SetErrorEngine(error)]
                }
                description::UiDescriptionEvent::CloseDescription => {
                    return vec![message::MainMessage::SetRootEngine]
                }
                description::UiDescriptionEvent::FollowUrl(url) => {
                    return vec![message::MainMessage::SetLoadDescriptionEngine(
                        url,
                        None,
                        None,
                        Some(self.ui_description.clone()),
                        Some(self.ui_description_state.clone()),
                    )]
                }
                description::UiDescriptionEvent::ValidateFormInQuery(url) => {
                    let data = self.ui_description_state.collect_form_data();
                    return vec![message::MainMessage::SetLoadDescriptionEngine(
                        url,
                        Some(data),
                        None,
                        Some(self.ui_description.clone()),
                        Some(self.ui_description_state.clone()),
                    )];
                }
                description::UiDescriptionEvent::ValidateFormInBody(url) => {
                    let data = self.ui_description_state.collect_form_data();
                    return vec![message::MainMessage::SetLoadDescriptionEngine(
                        url,
                        None,
                        Some(data),
                        Some(self.ui_description.clone()),
                        Some(self.ui_description_state.clone()),
                    )];
                }
                description::UiDescriptionEvent::SetDescriptionUi(mut new_description) => {
                    new_description.loading = false;
                    self.ui_description = *new_description;
                    self.ui_description_state = description::UiDescriptionState::default();
                }
            }
        }

        vec![]
    }
}
