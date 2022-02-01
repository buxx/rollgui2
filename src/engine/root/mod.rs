use quad_net::http_request::{Request, RequestBuilder};

use crate::message;

use super::Engine;

pub mod state;
pub mod ui;

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

    fn manage_do_login(&mut self) -> bool {
        if let Some(do_login_request) = self.do_login_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match data {
                    Ok(_data) => {
                        println!("OK!");
                        self.state.error_message = None
                    }
                    Err(error) => self.state.error_message = Some(format!("Error : {}", error)),
                }

                self.do_login_request = None;
            }
            return true;
        }

        return false;
    }
}

impl Engine for RootScene {
    fn run(&mut self) -> Option<message::MainMessage> {
        let loading = self.manage_do_login();

        if let Some(event) = ui::ui(&mut self.state, loading) {
            match event {
                ui::RootUiEvent::QuitGame => {
                    return Some(message::MainMessage::Quit);
                }
                ui::RootUiEvent::OpenZone => {
                    return Some(message::MainMessage::SetZoneEngine);
                }
                ui::RootUiEvent::DoLogin => {
                    self.do_login_request = Some(RequestBuilder::new("http://bux.fr").send());
                }
            }
        }

        self.state.first_frame = false;
        None
    }
}
