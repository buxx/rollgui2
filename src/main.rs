use macroquad::prelude::*;

pub mod engine;
pub mod message;

#[macroquad::main("RollGui2")]
async fn main() {
    let mut current_scene: Box<dyn engine::Engine> = Box::new(engine::root::RootScene::new());

    loop {
        clear_background(BLACK);

        if let Some(main_message) = current_scene.run() {
            match main_message {
                message::MainMessage::Quit => return,
                message::MainMessage::SetZoneEngine => {
                    current_scene = Box::new(engine::zone::ZoneEngine::new());
                }
                message::MainMessage::SetRootEngine => {
                    current_scene = Box::new(engine::root::RootScene::new());
                }
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
