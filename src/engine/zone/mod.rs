use macroquad::prelude::*;

use crate::{graphics, message};

use super::Engine;

pub mod builder;
pub mod scene;
pub mod state;
pub mod ui;

pub struct ZoneEngine {
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
}

impl ZoneEngine {
    pub fn new(graphics: graphics::Graphics, state: state::ZoneState) -> Self {
        Self { graphics, state }
    }

    fn inputs(&mut self) {
        // FIXME: player moves depending on the zone/tiles
        // Player movements
        if is_key_down(KeyCode::Down) {
            self.state.player.display_y += 1.;
        }
        if is_key_down(KeyCode::Up) {
            self.state.player.display_y -= 1.;
        }
        if is_key_down(KeyCode::Left) {
            self.state.player.display_x -= 1.;
        }
        if is_key_down(KeyCode::Right) {
            self.state.player.display_x += 1.;
        }
    }

    fn camera(&self) {
        let zoom_x = (self.state.map.concrete_width / screen_width()) * 2.;
        let zoom_y = (self.state.map.concrete_height / screen_height()) * 2.;

        let offset_x = self.state.player.display_x - screen_width() / 2.;
        let offset_y = self.state.player.display_y - screen_height() / 2.;

        let player_x = self.state.player.display_x;
        let player_y = self.state.player.display_x;
        let map_concrete_width = self.state.map.concrete_width;
        let map_concrete_height = self.state.map.concrete_height;

        set_camera(&Camera2D {
            zoom: Vec2::new(zoom_x, zoom_y),
            offset: Vec2::new(0., 0.),
            // offset: Vec2::new(-2.45, -3.2),
            // rotation: 180.,
            ..Default::default()
        });
    }
}

impl Engine for ZoneEngine {
    fn run(&mut self) -> Option<message::MainMessage> {
        self.inputs();
        self.camera();

        // Game
        scene::scene(&self.graphics, &self.state);

        // Ui
        set_default_camera();
        if let Some(event) = ui::ui(&self.state) {
            match event {
                ui::ZoneUiEvent::ReturnToRoot => {
                    return Some(message::MainMessage::SetRootEngine);
                }
            }
        }

        None
    }
}
