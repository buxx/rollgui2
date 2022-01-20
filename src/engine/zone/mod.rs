use crate::message;

use super::Engine;

pub mod scene;
pub mod state;
pub mod ui;

pub struct ZoneEngine {
    pub zone_state: state::ZoneState,
}

impl ZoneEngine {
    pub fn new() -> Self {
        Self {
            zone_state: state::ZoneState::new(),
        }
    }
}

impl Engine for ZoneEngine {
    fn run(&mut self) -> Option<message::MainMessage> {
        // Game
        scene::scene(&self.zone_state);

        // Ui
        if let Some(event) = ui::ui(&self.zone_state) {
            match event {
                ui::ZoneUiEvent::ReturnToRoot => {
                    return Some(message::MainMessage::SetRootEngine);
                }
            }
        }

        None
    }
}
