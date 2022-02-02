use crate::{client, engine::root::util::auth_failed, message};
use macroquad::prelude::*;
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
            state: state::RootState::new("bux", ""),
            do_login_request: None,
        }
    }

    fn manage_do_login(&mut self) -> Vec<RootEvent> {
        let mut events = vec![];

        if let Some(do_login_request) = self.do_login_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match auth_failed(data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            events.push(RootEvent::OpenZone);
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
    fn run(&mut self) -> Option<message::MainMessage> {
        let mut events = vec![];

        events.extend(self.manage_do_login());
        events.extend(ui::ui(&mut self.state));

        for event in events {
            match event {
                RootEvent::QuitGame => {
                    return Some(message::MainMessage::Quit);
                }
                RootEvent::OpenZone => {
                    return Some(message::MainMessage::SetZoneEngine);
                }
                RootEvent::DoLogin => {
                    let request = client::Client::get_current_character_id_request(
                        &self.state.login,
                        &self.state.password,
                    );
                    self.do_login_request = Some(request);
                    self.state.loading = true;
                }
            }
        }

        self.state.first_frame = false;
        None
    }
}

pub enum RootEvent {
    QuitGame,
    OpenZone,
    DoLogin,
}
