use crate::{graphics, message};

use super::Engine;

pub mod builder;
pub mod draw;
pub mod scene;
pub mod state;
pub mod ui;

pub struct ZoneEngine {
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
}

impl ZoneEngine {
    pub fn new(graphics: graphics::Graphics, state: state::ZoneState) -> Self {
        Self { graphics, state }
    }
}

impl Engine for ZoneEngine {
    fn run(&mut self) -> Option<message::MainMessage> {
        // Game
        scene::scene(&self.graphics, &self.state);

        // Ui
        if let Some(event) = ui::ui(&self.state) {
            match event {
                ui::ZoneUiEvent::ReturnToRoot => {
                    return Some(message::MainMessage::SetRootEngine);
                }
            }
        }

        None
    }
}
