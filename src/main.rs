use macroquad::prelude::*;

pub mod action;
pub mod animation;
pub mod client;
pub mod config;
pub mod description;
pub mod engine;
pub mod entity;
pub mod event;
pub mod graphics;
pub mod hardcoded;
pub mod message;
pub mod tileset;
pub mod ui;
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
                message::MainMessage::SetLoadDescriptionEngine(
                    url,
                    query,
                    data,
                    previous_ui_description,
                    previous_ui_description_state,
                    client,
                ) => {
                    let request = if let Some(client_) = &client {
                        client_.get_description_request(url, query, data)
                    } else {
                        client::Client::get_anonymous_description_request(&url, query, data)
                    };
                    current_scene = Box::new(engine::load_description::LoadDescriptionEngine::new(
                        request,
                        client,
                        previous_ui_description,
                        previous_ui_description_state,
                    ));
                }
                message::MainMessage::SetDescriptionEngine(description, client) => {
                    current_scene = Box::new(engine::description::DescriptionEngine::new(
                        description,
                        client,
                    ));
                }
                message::MainMessage::SetDescriptionEngineFrom(
                    ui_description,
                    ui_description_state,
                    client,
                ) => {
                    current_scene = Box::new(engine::description::DescriptionEngine::from_state(
                        ui_description,
                        ui_description_state,
                        client,
                    ));
                }
                message::MainMessage::AccountCreated => {
                    current_scene = Box::new(engine::root::RootScene::with_home_message(
                        "Compté créé, identifiez-vous".to_string(),
                        Some(egui::Color32::GREEN),
                    ));
                }
                message::MainMessage::CharacterCreated(login, password, character_id) => {
                    current_scene = Box::new(engine::load_zone::LoadZoneEngine::new(
                        graphics.clone(),
                        &login,
                        &password,
                        &character_id,
                    )?);
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
