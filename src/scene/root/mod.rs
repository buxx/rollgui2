use crate::{event, message};

use super::Scene;

pub mod scene;
pub mod ui;

pub struct RootScene {}

impl Scene for RootScene {
    fn run(&mut self) -> Option<message::MainMessage> {
        // Game
        scene::scene();

        // Ui
        if let Some(ui_event) = ui::ui() {
            match ui_event {
                event::UiEvent::QuitGame => {
                    return Some(message::MainMessage::Quit);
                }
            }
        }

        None
    }
}
