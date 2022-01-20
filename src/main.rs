use macroquad::prelude::*;

pub mod event;
pub mod message;
pub mod scene;

#[macroquad::main("RollGui2")]
async fn main() {
    let mut current_scene: Box<dyn scene::Scene> = Box::new(scene::root::RootScene {});

    loop {
        clear_background(BLACK);

        if let Some(main_message) = current_scene.run() {
            match main_message {
                message::MainMessage::Quit => return,
                message::MainMessage::ChangeScene(new_scene) => {
                    current_scene = new_scene;
                }
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
