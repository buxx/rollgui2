use crate::message;

use super::Engine;

pub mod scene;
pub mod state;
pub mod ui;

pub struct RootScene {
    state: state::RootState,
}

impl RootScene {
    pub fn new() -> Self {
        Self {
            state: state::RootState::new(),
        }
    }
}

impl Engine for RootScene {
    fn run(&mut self) -> Option<message::MainMessage> {
        // Game
        scene::scene(&self.state);

        // Ui
        if let Some(event) = ui::ui(&self.state) {
            match event {
                ui::RootUiEvent::QuitGame => {
                    return Some(message::MainMessage::Quit);
                }
                ui::RootUiEvent::ChangeHelloText(new_hello_text) => {
                    self.state.hello_text = new_hello_text;
                }
                ui::RootUiEvent::OpenZone => {
                    return Some(message::MainMessage::SetZoneEngine);
                }
            }
        }

        None
    }
}
