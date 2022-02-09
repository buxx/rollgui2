use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

use crate::{action, config, event as base_event, graphics, message, util as base_util};

use super::Engine;
use crate::gui;

// pub mod event;
pub mod event;
pub mod scene;
pub mod socket;
pub mod state;
pub mod ui;
pub mod util;

const DEFAULT_PLAYER_VELOCITY_DIVIDER: f32 = 2.5;
const DEFAULT_PLAYER_VELOCITY_LIMIT: f32 = 2.0;
const RUNNING_PLAYER_VELOCITY_LIMIT: f32 = 5.0;
const LEFT_PANEL_WIDTH: i32 = 250;
const QUICK_ACTION_MARGIN: f32 = 10.;

pub struct ZoneEngine {
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
    pub socket: WebSocket,
    pub socket_is_new: bool,
    pub tick_last: f64,
    pub tick_i: i16,
    pub zoom_mode: ZoomMode,
    pub last_limited_user_input: f64,
    pub disable_all_user_input_until: f64,
    pub disable_all_user_input: bool,
    pub user_inputs: Vec<UserInput>,
    pub running_mode: bool,
    pub last_require_around_coordinate: (i32, i32),
    pub quick_actions: Vec<action::quick::QuickAction>,
    pub selected_quick_action: Option<usize>,
    pub current_action: Option<action::Action>,
    pub pending_exploitable_tiles: Vec<usize>,
    pub mouse_zone_position: Vec2,
}

impl ZoneEngine {
    pub fn new(graphics: graphics::Graphics, state: state::ZoneState) -> Result<Self, String> {
        let socket = socket::get_socket(&state)?;
        Ok(Self {
            graphics,
            state,
            socket,
            socket_is_new: true,
            tick_last: get_time(),
            tick_i: 0,
            zoom_mode: ZoomMode::Normal,
            last_limited_user_input: get_time(),
            disable_all_user_input_until: get_time(),
            disable_all_user_input: false,
            user_inputs: vec![],
            running_mode: false,
            last_require_around_coordinate: (0, 0),
            quick_actions: vec![],
            selected_quick_action: None,
            current_action: None,
            pending_exploitable_tiles: vec![],
            mouse_zone_position: Vec2::new(0., 0.),
        })
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
                UserInput::SwitchRunningMode => self.running_mode = !self.running_mode,
                UserInput::InRunningMode => self.running_mode = true,
                UserInput::InWalkingMode => self.running_mode = false,
            }
        }

        // Update player velocity and limit its maximum speed
        let player_velocity_limit = if self.running_mode {
            RUNNING_PLAYER_VELOCITY_LIMIT
        } else {
            DEFAULT_PLAYER_VELOCITY_LIMIT
        };
        self.state.player_display.velocity += player_acceleration;
        if self.state.player_display.velocity.length() > player_velocity_limit {
            self.state.player_display.velocity =
                self.state.player_display.velocity.normalize() * player_velocity_limit;
        }

        // Update player position according to its velocity
        self.state.player_display.position += self.state.player_display.velocity;

        // Update player running animation
        let was_running = self.state.player_display.running.is_some();
        if self.state.player_display.velocity.length() > 0.05 {
            player_running = if self.state.player_display.velocity.y < -0.05 {
                Some(PlayerRunning::Top)
            } else if self.state.player_display.velocity.y > 0.05 {
                Some(PlayerRunning::Down)
            } else if self.state.player_display.velocity.x > 0.05 {
                Some(PlayerRunning::Right)
            } else if self.state.player_display.velocity.x < -0.05 {
                Some(PlayerRunning::Left)
            } else {
                None
            };
        }

        if was_running && player_running.is_none() {
            let coordinates = (self.state.player.zone_row_i, self.state.player.zone_col_i);
            if coordinates != self.last_require_around_coordinate {
                let event = util::require_around_event(&self.state);
                self.socket.send_text(&event);
                self.last_require_around_coordinate = coordinates;
            }
        }

        self.state.player_display.running = player_running;

        // Update player zone coordinates if changed
        let half_size_width = self.graphics.tile_width / 2.;
        let half_size_height = self.graphics.tile_height / 2.;
        let player_center_x = self.state.player_display.position.x + half_size_width;
        let player_center_y = self.state.player_display.position.y + half_size_height;

        let current_player_row_i = (player_center_y / self.graphics.tile_height) as i32;
        let current_player_col_i = (player_center_x / self.graphics.tile_width) as i32;

        if current_player_row_i != self.state.player.zone_row_i
            || current_player_col_i != self.state.player.zone_col_i
        {
            self.state.player.zone_row_i = current_player_row_i;
            self.state.player.zone_col_i = current_player_col_i;
            let player_move_event = util::player_move_event(&self.state);
            self.socket.send_text(&player_move_event);
        }
    }

    fn recv(&mut self) -> Vec<message::MainMessage> {
        while let Some(data) = self.socket.try_recv() {
            match base_event::ZoneEvent::from_u8(data) {
                Ok(event) => self.event(event),
                Err(error) => return vec![message::MainMessage::SetErrorEngine(error)],
            }
        }

        vec![]
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
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            self.user_inputs.push(UserInput::InRunningMode);
        }
        if is_key_released(KeyCode::LeftShift) || is_key_released(KeyCode::RightShift) {
            self.user_inputs.push(UserInput::InWalkingMode);
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
            // Avoid player move by click if currently in action
            if self.current_action.is_none() {
                let (pixels_x, pixels_y) = mouse_position();
                let position_local = base_util::convert_to_local(Vec2::new(pixels_x, pixels_y));
                self.user_inputs
                    .push(UserInput::MovePlayerBy(position_local * 2.0));
            }
        }
    }

    fn camera(&mut self) {
        let zoom_multiplier = match self.zoom_mode {
            ZoomMode::Large => 1.,
            ZoomMode::Normal => 2.,
            ZoomMode::Double => 4.,
        };

        let zoom_x = (self.state.map.concrete_width / screen_width()) * zoom_multiplier;
        let zoom_y = (self.state.map.concrete_height / screen_height()) * zoom_multiplier;
        let zoom = Vec2::new(zoom_x, zoom_y);

        let target_x = self.state.player_display.position.x / self.state.map.concrete_width;
        // Invert Y axis because the camera is Y inverted
        let target_y = -(self.state.player_display.position.y / self.state.map.concrete_height);
        let target = Vec2::new(target_x, target_y);

        set_camera(&Camera2D {
            zoom: zoom,
            target: target,
            ..Default::default()
        });

        let mut mouse_zone_position = mouse_position_local() / zoom;
        mouse_zone_position.x += target.x;
        mouse_zone_position.y -= target.y;
        self.mouse_zone_position = mouse_zone_position;
    }

    pub fn scene(&self) {
        scene::scene(&self.graphics, &self.state, self.tick_i);
    }

    pub fn draw_left_panel(&self) {
        gui::panel::draw_panel_background(&self.graphics);
    }

    fn draw_quick_actions(&mut self, action_clicked: bool) {
        let start_draw_x = LEFT_PANEL_WIDTH as f32 + QUICK_ACTION_MARGIN;
        let start_draw_y = screen_height() - gui::quick::BUTTON_HEIGHT - QUICK_ACTION_MARGIN;
        let mut quick_action_just_clicked = false;

        for (i, quick_action) in self.quick_actions.iter().enumerate() {
            let decal = i as f32 * (gui::quick::BUTTON_WIDTH + QUICK_ACTION_MARGIN);
            let draw_x = start_draw_x + decal;
            let draw_y = start_draw_y;
            // TODO : optimize ?
            let tile_id = self
                .graphics
                .find_tile_id_from_classes(&quick_action.classes);

            let active = if let Some(selected_quick_action) = self.selected_quick_action {
                selected_quick_action == i
            } else {
                false
            };

            if gui::quick::draw_quick_action_button(
                &self.graphics,
                active,
                &tile_id,
                draw_x,
                draw_y,
                self.tick_i,
            ) {
                if base_util::mouse_clicked() {
                    self.current_action = Some(action::Action::from_quick_action(&quick_action));
                    self.selected_quick_action = Some(i);
                    self.pending_exploitable_tiles = vec![];
                    quick_action_just_clicked = true;
                }
                self.disable_all_user_input = true;
            }
        }

        if base_util::mouse_clicked() && !quick_action_just_clicked && !action_clicked {
            self.selected_quick_action = None;
            self.current_action = None;
            self.pending_exploitable_tiles = vec![];
        }
    }

    fn draw_current_action(&mut self) -> bool {
        let mut exploitable_tile_clicked: Option<usize> = None;

        if let Some(current_action) = &self.current_action {
            for (i, exploitable_tile) in current_action.exploitable_tiles.iter().enumerate() {
                let exploitable_tile_blinking = self.pending_exploitable_tiles.contains(&i);
                if gui::action::draw_action_tile_in_camera(
                    &self.graphics,
                    &self.state,
                    exploitable_tile,
                    self.tick_i,
                    self.mouse_zone_position,
                    exploitable_tile_blinking,
                ) {
                    if base_util::mouse_clicked() {
                        exploitable_tile_clicked = Some(i);
                        info!("Action tile clicked");
                    }
                }
            }

            if let Some(exploitable_tile_clicked_) = exploitable_tile_clicked {
                if current_action.all_tiles_at_once {
                    self.pending_exploitable_tiles =
                        (0..current_action.exploitable_tiles.len()).collect();
                } else {
                    self.pending_exploitable_tiles = vec![exploitable_tile_clicked_];
                }
            }
        }

        exploitable_tile_clicked.is_some()
    }

    pub fn draw_buttons(&mut self) {
        let zoom_in = self.zoom_mode == ZoomMode::Double;
        if gui::button::draw_zoom_button(&self.graphics, zoom_in) {
            if base_util::mouse_clicked() {
                match self.zoom_mode {
                    ZoomMode::Normal => self.user_inputs.push(UserInput::ZoomIn),
                    ZoomMode::Large => self.user_inputs.push(UserInput::ZoomIn),
                    ZoomMode::Double => self.user_inputs.push(UserInput::ZoomOut),
                }
                self.disable_all_user_input_until = get_time() + 0.25;
            }

            self.disable_all_user_input = true;
        }

        if gui::button::draw_run_button(&self.graphics, self.running_mode) {
            if base_util::mouse_clicked() {
                self.user_inputs.push(UserInput::SwitchRunningMode);
                self.disable_all_user_input_until = get_time() + 0.25;
            }

            self.disable_all_user_input = true;
        }
    }

    fn manage_fresh_socket(&mut self) -> bool {
        if self.socket_is_new {
            // Socket just connected
            if self.socket.connected() {
                self.socket_is_new = false;
                let event = util::require_around_event(&self.state);
                self.socket.send_text(&event);
                let new_coordinates = (self.state.player.zone_row_i, self.state.player.zone_col_i);
                self.last_require_around_coordinate = new_coordinates;
                return false;
            }

            // Indicate to do nothing while socket is not connected
            return true;
        }
        return false;
    }
}

impl Engine for ZoneEngine {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        // wasm web socket connection must be awaited
        if self.manage_fresh_socket() {
            return vec![];
        }

        let mut messages = vec![];

        self.update_tick_i();
        self.user_inputs();
        self.update();
        messages.extend(self.recv());
        self.camera();

        // Game
        self.scene();
        let action_clicked = self.draw_current_action();

        // Ui
        set_default_camera();
        self.disable_all_user_input = false;
        self.draw_left_panel();
        self.draw_quick_actions(action_clicked);
        self.draw_buttons();
        if let Some(event) = ui::ui(&self.state) {
            match event {
                ui::ZoneUiEvent::ReturnToRoot => {
                    messages.push(message::MainMessage::SetRootEngine);
                }
            }
        }

        messages
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
    SwitchRunningMode,
    InRunningMode,
    InWalkingMode,
}

#[derive(PartialEq)]
pub enum ZoomMode {
    Normal,
    Double,
    Large,
}
