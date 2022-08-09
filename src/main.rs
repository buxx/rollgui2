use macroquad::prelude::*;
use structopt::StructOpt;
use ui::utils::egui_scale;

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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(short, long)]
    login: Option<String>,

    #[structopt(short, long)]
    password: Option<String>,
}

const SERVER_ADDRESS: &'static str = "http://127.0.0.1:5000";
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
    let opt = Opt::from_args();
    let mut current_scene: Box<dyn engine::Engine> = Box::new(engine::root::RootScene::new(&opt));
    // FIXME : manage errors
    let tile_set = load_texture("static/graphics.png").await.unwrap();
    let tile_set_bytes = load_file("static/graphics.png").await.unwrap();
    let tiles_mapping = tileset::loader::from_list(hardcoded::get_tiles_list(), 32., 32.);
    let mut graphics = graphics::Graphics::new(tile_set, tile_set_bytes, tiles_mapping, 32., 32.);

    // Set egui scale
    egui_macroquad::egui_mq_cfg(|equi_mq| {
        equi_mq.egui_input().pixels_per_point = Some(egui_scale())
    });

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
                    current_scene = Box::new(engine::root::RootScene::new(&opt));
                }
                message::MainMessage::SetErrorEngine(error_message) => {
                    current_scene = Box::new(engine::error::ErrorEngine::new(error_message));
                }
                message::MainMessage::SetZoneEngine(client, state) => {
                    let mut graphics = graphics.clone();
                    graphics = match graphics::utils::fill_avatars_from_zone_state(&state, graphics)
                        .await
                    {
                        Ok(graphics_) => graphics_,
                        Err(error) => {
                            current_scene = Box::new(engine::error::ErrorEngine::new(error));
                            continue;
                        }
                    };

                    match engine::zone::ZoneEngine::new(client, graphics, state) {
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
                }
            }
        }

        next_frame().await
    }
}
