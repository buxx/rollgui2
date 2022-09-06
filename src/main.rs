use default_env::default_env;
use engine::world::WorldEngine;
use macroquad::prelude::*;
use util::texture_from_cache_or_from_file;

use crate::{
    ui::utils::{egui_scale, loaded},
    util::{set_auth_token, set_remember_me, vname},
};

#[cfg(target_arch = "wasm32")]
use crate::ui::utils::reload_page;

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
pub mod media;
pub mod message;
pub mod tileset;
pub mod types;
pub mod ui;
pub mod util;
pub mod zone;

const SERVER_ADDRESS: &'static str = default_env!("SERVER_ADDRESS", "http://127.0.0.1:5000");
const VERSION: &str = default_env!("CARGO_PKG_VERSION", "0.1.0");

fn window_conf() -> Conf {
    Conf {
        window_title: "Rolling".to_owned(),
        window_resizable: true,
        window_width: 1280,
        window_height: 800,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Start rollgui2 ({})", VERSION);
    // FIXME : manage errors
    let graphics_name = vname("static/graphics.png");
    let tile_set = load_texture(&graphics_name).await.unwrap();
    let tile_set_bytes = load_file(&graphics_name).await.unwrap();
    let tiles_mapping = tileset::loader::from_list(hardcoded::get_tiles_list(), 32., 32.);
    let mut graphics = graphics::Graphics::new(tile_set, tile_set_bytes, tiles_mapping, 32., 32.);

    let root_illustration_name = vname("root.png");
    info!("Load root illustration {}", root_illustration_name);
    graphics.load_illustration(&root_illustration_name).await;

    let mut current_scene: Box<dyn engine::Engine> =
        Box::new(engine::root::RootScene::new(graphics.clone()));

    // Set egui scale
    egui_macroquad::ui(|_mq_ctx, egui_ctx| {
        egui_ctx.set_pixels_per_point(egui_scale());
    });

    loaded();

    loop {
        clear_background(BLACK);
        let messages = current_scene.tick();

        for message in messages {
            match message {
                message::MainMessage::SetLoadZoneEngine(client, character_id) => {
                    current_scene = Box::new(engine::load_zone::LoadZoneEngine::new(
                        graphics.clone(),
                        client,
                        &character_id,
                    )?);
                }
                message::MainMessage::SetLoadZoneEngineWithClient(client, character_id) => {
                    current_scene = Box::new(engine::load_zone::LoadZoneEngine::new(
                        graphics.clone(),
                        client,
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
                        // FIXME : how ot cost ?
                        graphics.clone(),
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
                        graphics.clone(),
                    ));
                }
                message::MainMessage::CharacterCreated(client, character_id) => {
                    current_scene = Box::new(engine::load_zone::LoadZoneEngine::new(
                        graphics.clone(),
                        client,
                        &character_id,
                    )?);
                }
                message::MainMessage::SetRootEngine => {
                    current_scene = Box::new(engine::root::RootScene::new(graphics.clone()));
                }
                message::MainMessage::SetErrorEngine(error_message) => {
                    current_scene = Box::new(engine::error::ErrorEngine::new(error_message));
                }
                message::MainMessage::SetZoneEngine(client, state) => {
                    let player_avatar_uuid = state.player.private_avatar_uuid();
                    let player_avatar_texture = texture_from_cache_or_from_file(&format!(
                        "media/character_avatar__original__{}.png",
                        player_avatar_uuid
                    ))
                    .await?;
                    graphics.add_avatar_texture(player_avatar_uuid, player_avatar_texture);

                    match engine::zone::ZoneEngine::new(client, graphics.clone(), state) {
                        Ok(engine) => {
                            current_scene = Box::new(engine);
                        }
                        Err(error) => {
                            current_scene = Box::new(engine::error::ErrorEngine::new(error));
                        }
                    };
                }
                message::MainMessage::SetEngine(engine) => {
                    current_scene = engine;
                }
                message::MainMessage::Quit => return Ok(()),
                message::MainMessage::LoadIllustration(illustration_name) => {
                    info!("Load illustration {}", illustration_name);
                    graphics.load_illustration(&illustration_name).await;
                    current_scene.replace_graphics(graphics.clone());
                    current_scene.signal_illustration_loaded(&illustration_name);
                }
                message::MainMessage::SetWorldEngine(client, player) => {
                    current_scene = Box::new(WorldEngine::new(graphics.clone(), client, player))
                }
                message::MainMessage::Exit => {
                    set_remember_me(false);
                    set_auth_token(None);

                    #[cfg(target_arch = "wasm32")]
                    {
                        reload_page();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        current_scene = Box::new(engine::root::RootScene::new(graphics.clone()));
                    }
                }
            }
        }

        next_frame().await
    }
}
