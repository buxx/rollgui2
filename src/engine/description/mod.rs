use crate::{client, description, entity, graphics, message};

pub struct DescriptionEngine {
    pub client: Option<client::Client>, // Client means authenticated requests
    pub ui_description: description::UiDescription,
    pub ui_description_state: description::UiDescriptionState,
}

impl DescriptionEngine {
    pub fn new(
        description: entity::description::Description,
        graphics: graphics::Graphics,
        client: Option<client::Client>,
    ) -> Self {
        Self {
            ui_description: description::UiDescription::new(description, graphics, None),
            ui_description_state: description::UiDescriptionState {
                ..Default::default()
            },
            client,
        }
    }

    pub fn from_state(
        ui_description: description::UiDescription,
        ui_description_state: description::UiDescriptionState,
        client: Option<client::Client>,
    ) -> Self {
        Self {
            ui_description,
            ui_description_state,
            client,
        }
    }
}

impl super::Engine for DescriptionEngine {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        // Some checks ...
        if self.ui_description.description.account_created {
            return vec![message::MainMessage::AccountCreated];
        }
        if let Some(new_character_id) = &self.ui_description.description.new_character_id {
            let client = self.client.as_ref().expect("Client must be defined here");
            return vec![message::MainMessage::CharacterCreated(
                client.clone(),
                new_character_id.to_string(),
            )];
        }

        let mut event = None;

        egui_macroquad::ui(|_mq_ctx, egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                event = self
                    .ui_description
                    .draw(egui_ctx, ui, &mut self.ui_description_state);
            });
        });
        egui_macroquad::draw();

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
                        self.client.clone(),
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
                        self.client.clone(),
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
                        self.client.clone(),
                    )];
                }
                description::UiDescriptionEvent::SetDescriptionUi(mut new_description) => {
                    new_description.loading = false;
                    self.ui_description = *new_description;
                    self.ui_description_state = description::UiDescriptionState::default();
                }
                // Managed inside of UiDescription
                description::UiDescriptionEvent::TextEditFocused(_, _, _) => {}
                description::UiDescriptionEvent::RequireIllustrationLoad(illustration_name) => {
                    return vec![message::MainMessage::LoadIllustration(illustration_name)];
                }
            }
        }

        vec![]
    }

    fn signal_illustration_loaded(&mut self, _illustration_name: &str) {}

    fn replace_graphics(&mut self, graphics: crate::graphics::Graphics) {
        self.ui_description.graphics = graphics;
    }
}
