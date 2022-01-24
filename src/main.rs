use macroquad::prelude::*;

pub mod config;
pub mod engine;
pub mod entity;
pub mod graphics;
pub mod hardcoded;
pub mod message;
pub mod tileset;
pub mod zone;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "RollGui2")]
pub struct Opt {
    #[structopt(name = "config_file_path", default_value = "config.ini")]
    pub config_file_path: String,
}

#[macroquad::main("RollGui2")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let config_file_path = opt.config_file_path.clone();
    let mut current_scene: Box<dyn engine::Engine> = Box::new(engine::root::RootScene::new());
    let tile_set = load_texture("graphics.png").await.unwrap();
    let tiles_mapping = tileset::loader::from_list(hardcoded::get_tiles_list(), 32., 32.);
    let graphics = graphics::Graphics::new(tile_set, tiles_mapping, 32., 32.);
    let server_login: String = "".to_string();
    let server_password: String = "".to_string();

    loop {
        clear_background(BLACK);

        if let Some(main_message) = current_scene.run() {
            match main_message {
                message::MainMessage::Quit => return Ok(()),
                message::MainMessage::SetZoneEngine => {
                    let config = config::Config::from_config_file(
                        config_file_path.clone(),
                        server_login.clone(),
                        server_password.clone(),
                    )?;
                    let engine =
                        engine::zone::builder::build_zone_engine(graphics.clone(), config)?;
                    current_scene = Box::new(engine);
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
