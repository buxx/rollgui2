use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

use crate::{
    action as base_action, animation, client, config, description, entity, event as base_event,
    graphics, message, ui::utils::is_mobile, util as base_util,
};

use self::resume::CharacterResume;

use super::Engine;

pub mod action;
pub mod animations;
pub mod event;
pub mod gui;
pub mod inventory;
pub mod left_panel;
pub mod log;
pub mod resume;
pub mod scene;
pub mod socket;
pub mod state;
pub mod ui;
pub mod util;

const DEFAULT_PLAYER_VELOCITY_DIVIDER: f32 = 2.5;
const DEFAULT_PLAYER_VELOCITY_LIMIT: f32 = 2.0;
const RUNNING_PLAYER_VELOCITY_LIMIT: f32 = 5.0;
const LEFT_PANEL_WIDTH: f32 = 250.;
const QUICK_ACTION_MARGIN: f32 = 10.;
pub const DISPLAY_USER_LOG_COUNT: usize = 5;
const HELPER_TEXT_FONT_SIZE: f32 = 23.;

pub struct ZoneEngine {
    pub client: client::Client,
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
    pub pending_events: Vec<UserEvent>,
    pub socket: WebSocket,
    pub socket_is_new: bool,
    pub tick_last: f64,
    pub tick_i: i16,
    pub frame_i: i64,
    pub zoom_mode: ZoomMode,
    pub animations: Vec<Box<dyn animation::Animation>>,
    pub last_limited_user_input: f64,
    pub disable_all_user_input_until: f64,
    pub disable_all_user_input: bool,
    pub user_inputs: Vec<UserInput>,
    pub running_mode: bool,
    pub last_require_around_coordinate: (i32, i32),
    pub quick_actions: Vec<base_action::quick::QuickAction>,
    pub selected_quick_action: Option<usize>,
    pub current_action: Option<base_action::Action>,
    pub pending_exploitable_tiles: Vec<usize>,
    pub mouse_zone_position: Vec2,
    pub mouse_zone_coordinates: (usize, usize),
    pub quick_action_requests: Vec<quad_net::http_request::Request>,
    pub user_logs: Vec<log::UserLog>,
    pub helper_text: Option<String>,
    pub description_request: Option<quad_net::http_request::Request>,
    pub current_left_panel_button: Option<gui::panel::Button>,
    pub current_description: Option<description::UiDescription>,
    pub current_description_state: Option<description::UiDescriptionState>,
    pub inventory_request: Option<quad_net::http_request::Request>,
    pub inventory_drop_request: Option<quad_net::http_request::Request>,
    pub inventory: Option<inventory::Inventory>,
    pub inventory_state: Option<inventory::InventoryState>,
    pub last_begin_click_coordinates: Option<Vec2>,
    pub last_begin_click_coordinates_this_frame: Option<Vec2>,
    pub last_begin_click_was_in_egui: Option<bool>,
    pub highlight_tiles: Vec<(usize, usize)>,
    pub resume: Option<CharacterResume>,
}

impl ZoneEngine {
    pub fn new(
        client: client::Client,
        graphics: graphics::Graphics,
        state: state::ZoneState,
    ) -> Result<Self, String> {
        let socket = socket::get_socket(&state)?;
        let zoom_mode = if is_mobile() {
            ZoomMode::Double
        } else {
            ZoomMode::Normal
        };
        Ok(Self {
            client,
            graphics,
            state,
            pending_events: vec![],
            socket,
            socket_is_new: true,
            tick_last: get_time(),
            tick_i: 0,
            frame_i: 0,
            zoom_mode,
            animations: vec![],
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
            mouse_zone_coordinates: (0, 0),
            quick_action_requests: vec![],
            user_logs: vec![],
            helper_text: None,
            description_request: None,
            current_left_panel_button: None,
            current_description: None,
            current_description_state: None,
            inventory_request: None,
            inventory_drop_request: None,
            inventory: None,
            inventory_state: None,
            last_begin_click_coordinates: None,
            last_begin_click_coordinates_this_frame: None,
            last_begin_click_was_in_egui: None,
            highlight_tiles: vec![],
            resume: None,
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

    fn update_frame_i(&mut self) {
        self.frame_i += 1;
    }

    fn consume_events(&mut self) {
        while let Some(event) = self.pending_events.pop() {
            match event {
                UserEvent::InventoryItemDropped(zone_row_i, zone_col_i, post_base_url) => {
                    self.inventory_item_dropped(zone_row_i, zone_col_i, post_base_url);
                }
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

        let velocity_limiter = self.state.map.get_speed(
            self.state.player.zone_row_i as usize,
            self.state.player.zone_col_i as usize,
            &entity::tile::TransportMode::Walking,
        );
        self.state.player_display.velocity *= velocity_limiter;

        // Update player position according to its velocity
        let next_position = self.state.player_display.position + self.state.player_display.velocity;

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
        let next_player_center_x = next_position.x + half_size_width;
        let next_player_center_y = next_position.y + half_size_height;

        let next_player_row_i = (next_player_center_y / self.graphics.tile_height) as i32;
        let next_player_col_i = (next_player_center_x / self.graphics.tile_width) as i32;

        let tile_is_traversable = self.state.map.traversable(
            next_player_row_i as usize,
            next_player_col_i as usize,
            &entity::tile::TransportMode::Walking,
        );
        let possible_build_is_traversable = self.possible_build_is_traversable(
            next_player_row_i as usize,
            next_player_col_i as usize,
            &entity::tile::TransportMode::Walking,
        );
        if tile_is_traversable && possible_build_is_traversable {
            self.state.player_display.position = next_position;
            if next_player_row_i != self.state.player.zone_row_i
                || next_player_col_i != self.state.player.zone_col_i
            {
                self.state.player.zone_row_i = next_player_row_i;
                self.state.player.zone_col_i = next_player_col_i;
                let player_move_event = util::player_move_event(&self.state);
                self.socket.send_text(&player_move_event);
            }
        }

        // User logs
        if self.user_logs.len() > DISPLAY_USER_LOG_COUNT {
            self.user_logs.remove(0);
        }

        // Mouse infos
        if base_util::mouse_pressed() && self.last_begin_click_coordinates.is_none() {
            self.last_begin_click_coordinates = Some(Vec2::from(mouse_position()));
        }
        if base_util::mouse_clicked() {
            self.last_begin_click_coordinates_this_frame =
                self.last_begin_click_coordinates.clone();
            self.last_begin_click_coordinates = None;
        } else {
            self.last_begin_click_coordinates_this_frame = None;
        }
    }

    fn recv_events(&mut self) -> Vec<message::MainMessage> {
        while let Some(data) = self.socket.try_recv() {
            match base_event::ZoneEvent::from_u8(data) {
                Ok(event) => self.event(event),
                Err(error) => return vec![message::MainMessage::SetErrorEngine(error)],
            }
        }

        vec![]
    }

    fn proceed_quick_action_requests(&mut self) {
        let mut to_removes: Vec<usize> = vec![];

        for (i, request) in self.quick_action_requests.iter_mut().enumerate() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(description_string) => {
                        match entity::description::Description::from_string(&description_string) {
                            Ok(description) => {
                                let message = &description
                                    .quick_action_response
                                    .unwrap_or_else(|| description.title.unwrap_or("".to_string()));
                                let message_level = if description.is_quick_error {
                                    log::UserLogLevel::Error
                                } else {
                                    log::UserLogLevel::Info
                                };
                                info!("Quick action response ({}) : {}", &message_level, &message,);

                                self.user_logs
                                    .push(log::UserLog::new(message.clone(), message_level));

                                // Clean exploitable tile blinking
                                if let (Some(current_action), Some(action_uuid)) =
                                    (&self.current_action, &description.action_uuid)
                                {
                                    if &current_action.uuid == action_uuid {
                                        self.pending_exploitable_tiles = vec![];
                                    }
                                }

                                // Animate exploitable tile if have to
                                if let Some(exploited_tile_position) =
                                    description.exploitable_success
                                {
                                    if let Some(current_action) = &self.current_action {
                                        for (i, exploitable_tile) in
                                            current_action.exploitable_tiles.iter().enumerate()
                                        {
                                            if exploitable_tile.zone_row_i
                                                == exploited_tile_position.0
                                                && exploitable_tile.zone_col_i
                                                    == exploited_tile_position.1
                                            {
                                                self.pending_exploitable_tiles.retain(|x| x != &i);

                                                let tile_id =
                                                    self.graphics.find_tile_id_from_classes(
                                                        &exploitable_tile.classes,
                                                    );
                                                match animation::pop::TilePopAnimation::new(
                                                    &self.graphics,
                                                    &self.state.map,
                                                    &tile_id,
                                                    exploitable_tile.zone_row_i,
                                                    exploitable_tile.zone_col_i,
                                                ) {
                                                    Ok(animation) => {
                                                        self.animations.push(Box::new(animation))
                                                    }
                                                    Err(error) => {
                                                        error!(
                                                            "Error during pop animation : {}",
                                                            error
                                                        );
                                                    }
                                                };

                                                break;
                                            }
                                        }
                                    }
                                }

                                // Animate deposit tile if have to
                                if let Some(((zone_row_i, zone_col_i), classes)) =
                                    description.deposit_success
                                {
                                    let tile_id = self.graphics.find_tile_id_from_classes(&classes);
                                    match animation::drop::TileDropAnimation::new(
                                        &self.graphics,
                                        &self.state.map,
                                        &tile_id,
                                        zone_row_i,
                                        zone_col_i,
                                    ) {
                                        Ok(animation) => self.animations.push(Box::new(animation)),
                                        Err(error) => {
                                            error!("Error during deposit animation : {}", error);
                                        }
                                    };
                                }
                            }
                            Err(error) => {
                                error!(
                                    "Quick action response description decoding ERROR : {}",
                                    error
                                );
                            }
                        };
                        // Quick action probably changes now
                        let event = util::require_around_event(&self.state);
                        self.socket.send_text(&event);
                    }
                    Err(error) => {
                        error!("Quick action response ERROR : {}", error);
                    }
                };
                to_removes.push(i);
            }
        }

        // Remove finished requests
        to_removes.sort();
        to_removes.reverse();
        for request_i_to_remove in to_removes {
            self.quick_action_requests.remove(request_i_to_remove);
        }
    }

    fn proceed_description_requests(&mut self) {
        if let Some(request) = self.description_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(description_string) => {
                        match entity::description::Description::from_string(&description_string) {
                            Ok(description) => {
                                self.current_description = Some(description::UiDescription::new(
                                    description,
                                    // FIXME : how it cost ?
                                    self.graphics.clone(),
                                    self.current_description.clone(),
                                ));
                                self.current_description_state =
                                    Some(description::UiDescriptionState::default());
                            }
                            Err(error) => {
                                error!("Error while decoding description : {}", error);
                            }
                        };
                    }
                    Err(error) => {
                        error!("Error while requiring description : {}", error);
                    }
                }
                self.current_left_panel_button = None;
                self.description_request = None;
            }
        }
    }

    fn user_inputs(&mut self) {
        if self.disable_all_user_input_until > get_time()
            || self.disable_all_user_input
            || self.current_description.is_some()
            || self.inventory.is_some()
        {
            return;
        }

        // Keyboard inputs without repetition limitation
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Z) || is_key_down(KeyCode::W) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(0., -1.)));
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(0., 1.)));
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Q) || is_key_down(KeyCode::A) {
            self.user_inputs
                .push(UserInput::MovePlayerBy(Vec2::new(-1., 0.)));
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
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
        let (pixels_x, pixels_y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            if
            // Avoid player move by click if currently in action
            self.current_action.is_none()
            // Avoid player move if mouse hover left menu
            && pixels_x > LEFT_PANEL_WIDTH
            {
                let position_local = base_util::convert_to_local(Vec2::new(pixels_x, pixels_y));
                self.user_inputs
                    .push(UserInput::MovePlayerBy(position_local * 4.5));
            }
        }
    }

    fn camera(&mut self) -> ((i32, i32), (i32, i32)) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let zoom_multiplier = self.zoom_mode.factor();
        let zoom_x = (self.state.map.concrete_width / screen_width) * zoom_multiplier;
        let zoom_y = (self.state.map.concrete_height / screen_height) * zoom_multiplier;
        let zoom = Vec2::new(zoom_x, zoom_y);

        let target_x = self.state.player_display.position.x / self.state.map.concrete_width;
        // Invert Y axis because the camera is Y inverted
        let target_y = -(self.state.player_display.position.y / self.state.map.concrete_height);
        let target = Vec2::new(target_x, target_y);

        let tiles_in_width = (screen_width / self.graphics.tile_width) as i32;
        let tiles_in_height = (screen_height / self.graphics.tile_height) as i32;
        let player_col_i = (self.state.player_display.position.x / self.graphics.tile_width) as i32;
        let player_row_i =
            (self.state.player_display.position.y / self.graphics.tile_height) as i32;
        let start_area_col_i = player_col_i - tiles_in_width / 2;
        let start_area_row_i = player_row_i - tiles_in_height / 2;
        let displayed_area = (
            (start_area_row_i, start_area_col_i),
            (
                start_area_row_i + tiles_in_height + 1,
                start_area_col_i + tiles_in_width + 1,
            ),
        );

        set_camera(&Camera2D {
            zoom: zoom,
            target: target,
            ..Default::default()
        });

        let mut mouse_zone_position = mouse_position_local() / zoom;
        mouse_zone_position.x += target.x;
        mouse_zone_position.y -= target.y;
        self.mouse_zone_position = mouse_zone_position;

        let concrete_mouse_x = self.mouse_zone_position.x * self.state.map.concrete_width as f32;
        let concrete_mouse_y = self.mouse_zone_position.y * self.state.map.concrete_height as f32;
        let zone_row_i = (concrete_mouse_y / self.graphics.tile_height) as usize;
        let zone_col_i = (concrete_mouse_x / self.graphics.tile_width) as usize;
        self.mouse_zone_coordinates = (zone_row_i + 1, zone_col_i);

        displayed_area
    }

    pub fn scene(&self, draw_area: ((i32, i32), (i32, i32))) {
        scene::scene(&self.graphics, &self.state, self.tick_i, draw_area);
    }

    fn draw_zone_ux(&mut self) {
        while let Some((row_i, col_i)) = self.highlight_tiles.pop() {
            self.graphics.draw_tile_highlight(
                row_i,
                col_i,
                self.state.map.concrete_width,
                self.state.map.concrete_height,
            );
        }
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

    fn possible_build_is_traversable(
        &self,
        row_i: usize,
        col_i: usize,
        walking: &entity::tile::TransportMode,
    ) -> bool {
        if let Some(build) = self.state.builds.get(&(row_i as i32, col_i as i32)) {
            return *build.traversable.get(walking.to_string()).unwrap_or(&true);
        }

        true
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
        self.update_frame_i();
        self.consume_events();
        self.user_inputs();
        self.update();
        self.proceed_quick_action_requests();
        self.proceed_description_requests();
        self.proceed_inventory_requests();
        messages.extend(self.recv_events());
        let draw_area = self.camera();

        // Game
        self.scene(draw_area);
        self.animations();
        self.draw_zone_ux();
        let action_clicked = self.draw_current_action();

        // Ui
        set_default_camera();
        self.disable_all_user_input = false;
        self.draw_left_panel();
        self.draw_resume_items();
        self.draw_user_logs();
        self.draw_quick_actions(action_clicked);
        self.draw_buttons();
        self.draw_helper_text();
        self.draw_inventory();
        self.helper_text = None;

        messages.extend(self.ui());

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

impl ZoomMode {
    pub fn factor(&self) -> f32 {
        match self {
            ZoomMode::Large => 1.,
            ZoomMode::Normal => 2.,
            ZoomMode::Double => {
                if is_mobile() {
                    6.
                } else {
                    4.
                }
            }
        }
    }
}

pub enum UserEvent {
    // zone_row_i, zone_col_i, post_base_url
    InventoryItemDropped(usize, usize, String),
}
