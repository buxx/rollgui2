use macroquad::prelude::*;

pub mod engine;
pub mod graphics;
pub mod hardcoded;
pub mod message;
pub mod tileset;
pub mod zone;

#[macroquad::main("RollGui2")]
async fn main() {
    let mut current_scene: Box<dyn engine::Engine> = Box::new(engine::root::RootScene::new());
    let tile_set = load_texture("graphics.png").await.unwrap();
    let tiles_mapping = tileset::loader::from_list(hardcoded::get_tiles_list(), 32., 32.);
    let graphics = graphics::Graphics::new(tile_set, tiles_mapping);

    loop {
        clear_background(BLACK);

        if let Some(main_message) = current_scene.run() {
            match main_message {
                message::MainMessage::Quit => return,
                message::MainMessage::SetZoneEngine => {
                    let source = Vec::new();
                    let engine = engine::zone::builder::build_zone_engine(graphics.clone(), source);
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
