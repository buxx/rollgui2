use crate::event;
use crate::message;

pub mod root;

pub enum Scene {
    Root,
}

pub fn run_scene(scene: &Scene) -> Option<message::MainMessage> {
    let mut main_message = None;

    match scene {
        Scene::Root => {
            root::scene::scene();
            if let Some(ui_event) = root::ui::ui() {
                match ui_event {
                    event::UiEvent::QuitGame => {
                        main_message = Some(message::MainMessage::Quit);
                    }
                }
            }
        }
    }

    main_message
}
