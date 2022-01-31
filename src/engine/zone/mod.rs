use macroquad::prelude::*;

use crate::{config, graphics, message, util};

use super::Engine;
use crate::gui;

pub mod builder;
pub mod scene;
pub mod state;
pub mod ui;

const DEFAULT_PLAYER_VELOCITY_DIVIDER: f32 = 2.5;
const LEFT_PANEL_WIDTH: i32 = 250;

pub struct ZoneEngine {
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
    pub tick_last: f64,
    pub tick_i: i16,
    pub zoom_mode: ZoomMode,
    pub last_limited_user_input: f64,
    pub disable_all_user_input_until: f64,
    pub disable_all_user_input: bool,
    pub user_inputs: Vec<UserInput>,
}

impl ZoneEngine {
    pub fn new(graphics: graphics::Graphics, state: state::ZoneState) -> Self {
        Self {
            graphics,
            state,
            tick_last: get_time(),
            tick_i: 0,
            zoom_mode: ZoomMode::Normal,
            last_limited_user_input: get_time(),
            disable_all_user_input_until: get_time(),
            disable_all_user_input: false,
            user_inputs: vec![],
        }
    }

    fn update_tick_i(&mut self) {
        let now = get_time();
        if now - self.tick_last >= 0.166 {
            self.tick_i += 1;
            self.tick_last = now;
            if self.tick_i >= config::SPRITES_COUNT {
                self.tick_i = 0;
            }
        }
    }

    fn update(&mut self) {
        // Player movements
        // TODO: player moves depending on the zone tiles
        let mut player_acceleration =
            -self.state.player_display.velocity / DEFAULT_PLAYER_VELOCITY_DIVIDER;
        let mut player_running: Option<PlayerRunning> = None;

        while let Some(user_input) = self.user_inputs.pop() {
            match user_input {
                UserInput::MovePlayerBy(vector) => {
                    player_acceleration += vector;
                }
                UserInput::ZoomIn => {
                    self.zoom_mode = match self.zoom_mode {
                        ZoomMode::Normal => ZoomMode::Double,
                        ZoomMode::Double => ZoomMode::Double,
                        ZoomMode::Large => ZoomMode::Normal,
                    }
                }
                UserInput::ZoomOut => {
                    self.zoom_mode = match self.zoom_mode {
                        ZoomMode::Normal => ZoomMode::Large,
                        ZoomMode::Double => ZoomMode::Normal,
                        ZoomMode::Large => ZoomMode::Large,
                    }
                }
            }
        }

        // Update player velocity and limit its maximum speed
        self.state.player_display.velocity += player_acceleration;
        if self.state.player_display.velocity.length() > 2. {
            self.state.player_display.velocity =
                self.state.player_display.velocity.normalize() * 2.;
        }

        // Update player position according to its velocity
        self.state.player_display.position += self.state.player_display.velocity;

        // Update player running animation
        if self.state.player_display.velocity.length() > 0.05 {
            player_running = if player_acceleration.y < -0.05 {
                Some(PlayerRunning::Top)
            } else if player_acceleration.y > 0.05 {
                Some(PlayerRunning::Down)
            } else if player_acceleration.x > 0.05 {
                Some(PlayerRunning::Right)
            } else if player_acceleration.x < -0.05 {
                Some(PlayerRunning::Left)
            } else {
                None
            };
        }
        self.state.player_display.running = player_running;
    }

    fn user_inputs(&mut self) {
        if self.disable_all_user_input_until > get_time() || self.disable_all_user_input {
            return;
        }

        // Keyboard inputs without repetition limitation
        if is_key_down(KeyCode::Up) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(0., -1.)));
        }
        if is_key_down(KeyCode::Down) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(0., 1.)));
        }
        if is_key_down(KeyCode::Left) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(-1., 0.)));
        }
        if is_key_down(KeyCode::Right) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(1., 0.)));
        }

        // Keyboard inputs with repetition limitation
        if get_time() - self.last_limited_user_input > 0.5 {
            if is_key_down(KeyCode::I) {
                self.user_inputs.push(UserInput::ZoomIn);
                self.last_limited_user_input = get_time();
            }
            if is_key_down(KeyCode::O) {
                self.user_inputs.push(UserInput::ZoomOut);
                self.last_limited_user_input = get_time();
            }
        }

        // Mouse inputs
        if is_mouse_button_down(MouseButton::Left) {
            let (pixels_x, pixels_y) = mouse_position();
            let position_local = util::convert_to_local(Vec2::new(pixels_x, pixels_y));
            self.user_inputs
                .push(UserInput::MovePlayerBy(position_local * 2.0));
        }
    }

    fn camera(&self) {
        let zoom_multiplier = match self.zoom_mode {
            ZoomMode::Large => 1.,
            ZoomMode::Normal => 2.,
            ZoomMode::Double => 4.,
        };

        let zoom_x = (self.state.map.concrete_width / screen_width()) * zoom_multiplier;
        let zoom_y = (self.state.map.concrete_height / screen_height()) * zoom_multiplier;

        let target_x = self.state.player_display.position.x / self.state.map.concrete_width;
        // Invert Y axis because the camera is Y inverted
        let target_y = -(self.state.player_display.position.y / self.state.map.concrete_height);

        set_camera(&Camera2D {
            zoom: Vec2::new(zoom_x, zoom_y),
            target: Vec2::new(target_x, target_y),
            ..Default::default()
        });
    }

    pub fn scene(&self) {
        scene::scene(&self.graphics, &self.state, self.tick_i);
    }

    pub fn draw_left_panel(&self) {
        gui::panel::draw_panel_background(&self.graphics);
    }

    pub fn draw_buttons(&mut self) {
        if gui::button::draw_zoom_button(&self.graphics) {
            if util::mouse_clicked() {
                match self.zoom_mode {
                    ZoomMode::Normal => self.user_inputs.push(UserInput::ZoomIn),
                    ZoomMode::Large => self.user_inputs.push(UserInput::ZoomIn),
                    ZoomMode::Double => self.user_inputs.push(UserInput::ZoomOut),
                }
                self.disable_all_user_input_until = get_time() + 0.25;
            }

            self.disable_all_user_input = true;
        }
    }
}

impl Engine for ZoneEngine {
    fn run(&mut self) -> Option<message::MainMessage> {
        self.update_tick_i();

        self.user_inputs();
        self.update();

        self.camera();

        // Game
        self.scene();

        // Ui
        set_default_camera();

        self.disable_all_user_input = false;
        self.draw_left_panel();
        self.draw_buttons();
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

pub enum PlayerRunning {
    Top,
    Down,
    Right,
    Left,
}

pub enum UserInput {
    MovePlayerBy(Vec2),
    ZoomIn,
    ZoomOut,
}

pub enum ZoomMode {
    Normal,
    Double,
    Large,
}
