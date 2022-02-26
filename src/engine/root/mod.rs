use crate::{client, engine::root::util::auth_failed, message};
use quad_net::http_request::Request;

use super::Engine;

pub mod state;
pub mod ui;
pub mod util;

pub struct RootScene {
    state: state::RootState,
    do_login_request: Option<Request>,
}

impl RootScene {
    pub fn new() -> Self {
        Self {
            state: state::RootState::new("", ""),
            do_login_request: None,
        }
    }

    fn manage_do_login(&mut self) -> Vec<RootEvent> {
        let mut events = vec![];

        if let Some(do_login_request) = self.do_login_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match auth_failed(&data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            let character_id = &data.unwrap_or("".to_string());
                            if character_id == "" {
                                events.push(RootEvent::GoToCreateCharacter(
                                    self.state.login.clone(),
                                    self.state.password.clone(),
                                ));
                            } else {
                                events.push(RootEvent::GoToZone(
                                    self.state.login.clone(),
                                    self.state.password.clone(),
                                    character_id.clone(),
                                ));
                            }
                        }
                    }
                    Err(error) => {
                        self.state.error_message = Some(format!("Erreur : {}", error));
                    }
                }

                self.state.loading = false;
                self.do_login_request = None;
            }
        }

        events
    }
}

impl Engine for RootScene {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        let mut messages = vec![];
        let mut events = vec![];

        events.extend(self.manage_do_login());
        events.extend(ui::ui(&mut self.state));

        for event in events {
            match event {
                RootEvent::QuitGame => {
                    messages.push(message::MainMessage::Quit);
                }
                RootEvent::GoToZone(login, password, character_id) => {
                    messages.push(message::MainMessage::SetLoadZoneEngine(
                        login,
                        password,
                        character_id,
                    ));
                }
                RootEvent::GoToCreateCharacter(login, password) => {
                    messages.push(message::MainMessage::SetCreateCharacterEngine(
                        login, password,
                    ));
                }
                RootEvent::DoLogin => {
                    let request = client::Client::get_current_character_id_request(
                        &self.state.login,
                        &self.state.password,
                    );
                    self.do_login_request = Some(request);
                    self.state.loading = true;
                }
                RootEvent::GoToCreateAccount => {
                    messages.push(message::MainMessage::SetLoadDescriptionEngine(
                        "/account/create".to_string(),
                        None,
                        None,
                        None,
                        None,
                    ))
                }
            }
        }

        self.state.first_frame = false;
        messages
    }
}

pub enum RootEvent {
    QuitGame,
    GoToCreateCharacter(String, String),
    GoToCreateAccount,
    GoToZone(String, String, String),
    DoLogin,
}
