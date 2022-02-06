use macroquad::prelude::*;

pub mod client;
pub mod config;
pub mod engine;
pub mod entity;
pub mod graphics;
pub mod gui;
pub mod hardcoded;
pub mod message;
pub mod tileset;
pub mod util;
pub mod zone;

const SERVER_ADDRESS: &'static str = env!("SERVER_ADDRESS");

#[macroquad::main("RollGui2")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut current_scene: Box<dyn engine::Engine> = Box::new(engine::root::RootScene::new());
    let tile_set = load_texture("static/graphics.png").await.unwrap();
    let tiles_mapping = tileset::loader::from_list(hardcoded::get_tiles_list(), 32., 32.);
    let graphics = graphics::Graphics::new(tile_set, tiles_mapping, 32., 32.);

    loop {
        clear_background(BLACK);
        let messages = current_scene.tick();

        for message in messages {
            match message {
                message::MainMessage::SetLoadZoneEngine(login, password, character_id) => {
                    current_scene = Box::new(engine::load_zone::LoadZoneEngine::new(
                        graphics.clone(),
                        &login,
                        &password,
                        &character_id,
                    )?);
                }
                message::MainMessage::SetCreateCharacterEngine(login, password) => {
                    todo!();
                }
                message::MainMessage::SetRootEngine => {
                    current_scene = Box::new(engine::root::RootScene::new());
                }
                message::MainMessage::SetErrorEngine(error_message) => {
                    current_scene = Box::new(engine::error::ErrorEngine::new(error_message));
                }
                message::MainMessage::SetEngine(engine) => {
                    current_scene = engine;
                }
                message::MainMessage::Quit => return Ok(()),
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
