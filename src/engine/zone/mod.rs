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
        // Player movements
        // TODO: player moves depending on the zone tiles
        let rotation_radians = self.state.player_display.rotation.to_radians();
        let mut acceleration = -self.state.player_display.velocity / 2.5;

        if is_key_down(KeyCode::Up) {
            acceleration = Vec2::new(rotation_radians.sin(), -rotation_radians.cos()) / 3.;
        }
        if is_key_down(KeyCode::Down) {
            acceleration = Vec2::new(-rotation_radians.sin(), rotation_radians.cos()) / 3.;
        }
        if is_key_down(KeyCode::Left) {
            self.state.player_display.rotation -= 2.5;
        }
        if is_key_down(KeyCode::Right) {
            self.state.player_display.rotation += 2.5;
        }

        self.state.player_display.velocity += acceleration;
        if self.state.player_display.velocity.length() > 2. {
            self.state.player_display.velocity =
                self.state.player_display.velocity.normalize() * 2.;
        }
        self.state.player_display.position += self.state.player_display.velocity;

        if self.state.player_display.velocity.length() > 0.25 {
            self.state.player_display.moving = true;
        } else {
            self.state.player_display.moving = false;
        }
    }

    fn camera(&self) {
        let zoom_x = (self.state.map.concrete_width / screen_width()) * 2.;
        let zoom_y = (self.state.map.concrete_height / screen_height()) * 2.;

        let target_x = self.state.player_display.position.x / self.state.map.concrete_width;
        // Invert Y axis because the camera is Y inverted
        let target_y = -(self.state.player_display.position.y / self.state.map.concrete_height);

        set_camera(&Camera2D {
            zoom: Vec2::new(zoom_x, zoom_y),
            target: Vec2::new(target_x, target_y),
            // offset: Vec2::new(-2.45, -3.2),
            rotation: self.state.player_display.rotation,
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
