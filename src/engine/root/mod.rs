use ehttp;
use poll_promise::Promise;

use crate::message;

use super::Engine;

pub mod state;
pub mod ui;

pub struct RootScene {
    state: state::RootState,
    do_login_promise: Option<Promise<ehttp::Result<ehttp::Response>>>,
}

impl RootScene {
    pub fn new() -> Self {
        Self {
            state: state::RootState::new("bux", ""),
            do_login_promise: None,
        }
    }

    fn manage_do_login_promise(&mut self) -> bool {
        if let Some(promise) = &self.do_login_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(resource) => {
                        println!("OK!");
                    }
                    Err(error) => {
                        self.state.error_message = Some(format!("Erreur : {}", error));
                    }
                };
                self.do_login_promise = None;
            } else {
                return true;
            }
        }

        return false;
    }
}

impl Engine for RootScene {
    fn run(&mut self) -> Option<message::MainMessage> {
        let loading = self.manage_do_login_promise();

        if let Some(event) = ui::ui(&mut self.state, loading) {
            match event {
                ui::RootUiEvent::QuitGame => {
                    return Some(message::MainMessage::Quit);
                }
                ui::RootUiEvent::OpenZone => {
                    return Some(message::MainMessage::SetZoneEngine);
                }
                ui::RootUiEvent::DoLogin => {
                    let (sender, promise) = Promise::new();
                    let request = ehttp::Request::get("https://bux.fr");
                    ehttp::fetch(request, move |response| sender.send(response));
                    self.do_login_promise = Some(promise);
                }
            }
        }

        self.state.first_frame = false;
        None
    }
}
