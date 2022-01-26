use std::time::{Duration, SystemTime};

use macroquad::prelude::*;

use crate::{config, graphics, message};

use super::Engine;

pub mod builder;
pub mod scene;
pub mod state;
pub mod ui;

pub struct ZoneEngine {
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
    pub tick_last: SystemTime,
    pub tick_i: i16,
}

impl ZoneEngine {
    pub fn new(graphics: graphics::Graphics, state: state::ZoneState) -> Self {
        Self {
            graphics,
            state,
            tick_last: SystemTime::now(),
            tick_i: 0,
        }
    }

    fn update_tick_i(&mut self) {
        if SystemTime::now().duration_since(self.tick_last).unwrap() >= Duration::from_millis(166) {
            self.tick_i += 1;
            self.tick_last = SystemTime::now();
            if self.tick_i >= config::SPRITES_COUNT {
                self.tick_i = 0;
            }
        }
    }

    fn inputs(&mut self) {
        // FIXME: player moves depending on the zone/tiles
        // Player movements
        if is_key_down(KeyCode::Down) {
            // self.state.player.display_y += 1.;
        }
        if is_key_down(KeyCode::Up) {
            let rotation_radians = self.state.player.display_rotation.to_radians();
            self.state.player.display_x += rotation_radians.sin();
            self.state.player.display_y -= rotation_radians.cos();
        }
        if is_key_down(KeyCode::Left) {
            self.state.player.display_rotation -= 5.;
        }
        if is_key_down(KeyCode::Right) {
            self.state.player.display_rotation += 5.;
        }
    }

    fn camera(&self) {
        let zoom_x = (self.state.map.concrete_width / screen_width()) * 2.;
        let zoom_y = (self.state.map.concrete_height / screen_height()) * 2.;

        let target_x = self.state.player.display_x / self.state.map.concrete_width;
        // Invert Y axis because the camera is Y inverted
        let target_y = -(self.state.player.display_y / self.state.map.concrete_height);

        set_camera(&Camera2D {
            zoom: Vec2::new(zoom_x, zoom_y),
            target: Vec2::new(target_x, target_y),
            // offset: Vec2::new(-2.45, -3.2),
            rotation: self.state.player.display_rotation,
            ..Default::default()
        });
    }

    pub fn scene(&self) {
        scene::scene(&self.graphics, &self.state, self.tick_i);
    }
}

impl Engine for ZoneEngine {
    fn run(&mut self) -> Option<message::MainMessage> {
        self.update_tick_i();

        self.inputs();
        self.camera();

        // Game
        self.scene();

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
