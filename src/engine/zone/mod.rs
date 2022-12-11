use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

use crate::{
    action as base_action, animation, client, config, description,
    engine::zone::util::live_message_event,
    entity::{self, description::RequestClicks},
    event as base_event, graphics,
    message::{self, MainMessage},
    ui::{
        text_input::TextInputRequest,
        utils::{egui_scale, is_mobile, open_url},
    },
    util::{self as base_util, mouse_clicked},
};

use self::{
    debug::DebugInfo,
    gui::{
        blink::BlinkingIcon, chat::display::Display as ChatDisplay, chat::state::State as ChatState,
    },
    resume::CharacterResume,
};

use super::Engine;

pub mod action;
pub mod animations;
pub mod blink;
pub mod click;
pub mod debug;
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

// This is a hack in regards of https://github.com/not-fl3/quad-net/issues/5
// Manage an unique websocket object for application lifetime
// TODO : Manage close and reopen when player will be able to disconnect
// (but no idea to how to close ws with wasm: https://github.com/not-fl3/quad-net/pull/4)
static mut WS: Option<WebSocket> = None;

fn web_socket(state: &state::ZoneState) -> &'static mut WebSocket {
    unsafe {
        if WS.is_none() {
            WS = Some(socket::get_socket(&state).unwrap());
        }
        WS.as_mut().unwrap()
    }
}

pub struct ZoneEngine {
    pub client: client::Client,
    pub graphics: graphics::Graphics,
    pub state: state::ZoneState,
    pub pending_events: Vec<UserEvent>,
    pub socket_is_new: bool,
    pub tick_last: f64,
    pub tick_i: i16,
    pub tick9_i: i16,
    pub frame_i: i64,
    pub zoom_mode: ZoomMode,
    pub camera_animations: Vec<Box<dyn animation::Animation>>,
    pub ui_animations: Vec<Box<dyn animation::Animation>>,
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
    pub blinking_icons: Vec<BlinkingIcon>,
    pub request_clicks: Option<RequestClicks>,
    pub pending_request_clicks: Option<(RequestClicks, i32, i32)>,
    pub click_begin_in_quick_action: Option<(f32, f32)>,
    pub quick_action_x_offset: Option<f32>,
    pub debug_info: DebugInfo,
    pub display_debug_info: bool,
    pub top_left_corner_click_counter: i32,
    chat_state: ChatState,
    chat_text_input_request: Option<TextInputRequest>,
}

impl ZoneEngine {
    pub fn new(
        client: client::Client,
        graphics: graphics::Graphics,
        state: state::ZoneState,
    ) -> Result<Self, String> {
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
            socket_is_new: true,
            tick_last: get_time(),
            tick_i: 0,
            tick9_i: 0,
            frame_i: 0,
            zoom_mode,
            camera_animations: vec![],
            ui_animations: vec![],
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
            blinking_icons: vec![],
            request_clicks: None,
            pending_request_clicks: None,
            click_begin_in_quick_action: None,
            quick_action_x_offset: None,
            debug_info: DebugInfo::new(),
            display_debug_info: false,
            top_left_corner_click_counter: 0,
            chat_state: ChatState::new(),
            chat_text_input_request: None,
        })
    }

    fn update_tick_i(&mut self) {
        let now = get_time();
        if now - self.tick_last >= 0.166 {
            self.tick_last = now;

            self.tick_i += 1;
            if self.tick_i >= config::SPRITES_COUNT {
                self.tick_i = 0;
            }

            self.tick9_i += 1;
            if self.tick9_i >= config::SPRITES_COUNT {
                self.tick9_i = 0;
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
        let mouse_position = mouse_position();
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
                UserInput::SubmitChatInput => {
                    info!("Send chat message");
                    web_socket(&self.state).send_text(&live_message_event(
                        &self.state.player.id,
                        self.chat_state.input_value().to_string(),
                    ));
                    self.chat_state.reset_input_value();
                    if !is_mobile() {
                        self.chat_state.set_request_focus();
                    }
                }
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
                web_socket(&self.state).send_text(&event);
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
                web_socket(&self.state).send_text(&player_move_event);
            }
        }

        // User logs
        if self.user_logs.len() > DISPLAY_USER_LOG_COUNT {
            self.user_logs.remove(0);
        }

        // Mouse infos
        if base_util::mouse_pressed() && self.last_begin_click_coordinates.is_none() {
            self.last_begin_click_coordinates = Some(Vec2::from(mouse_position.clone()));
        }
        if base_util::mouse_clicked() {
            self.last_begin_click_coordinates_this_frame =
                self.last_begin_click_coordinates.clone();
            self.last_begin_click_coordinates = None;
        } else {
            self.last_begin_click_coordinates_this_frame = None;
        }

        // Debug display
        if is_key_released(KeyCode::F12) {
            self.display_debug_info = !self.display_debug_info;
        }
        if mouse_clicked() {
            if mouse_position.0 <= 150.0 && mouse_position.1 <= 150.0 {
                self.top_left_corner_click_counter += 1;
            } else {
                self.top_left_corner_click_counter = 0;
            }
        }
        if self.top_left_corner_click_counter == 5 {
            self.display_debug_info = !self.display_debug_info;
            self.top_left_corner_click_counter = 0;
        }
    }

    fn recv_events(&mut self) -> Vec<message::MainMessage> {
        while let Some(data) = web_socket(&self.state).try_recv() {
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
                                if let Some(redirect) = description.redirect {
                                    self.description_request = Some(
                                        self.client.get_description_request(redirect, None, None),
                                    );
                                    self.quick_actions = vec![];
                                }

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
                                                    Ok(animation) => self
                                                        .camera_animations
                                                        .push(Box::new(animation)),
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
                                        Ok(animation) => {
                                            self.camera_animations.push(Box::new(animation))
                                        }
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
                        web_socket(&self.state).send_text(&event);
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

    fn proceed_description_requests(&mut self) -> Vec<message::MainMessage> {
        if let Some(request) = self.description_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(description_string) => {
                        match entity::description::Description::from_string(&description_string) {
                            Ok(description) => {
                                // If it is a clicks request, setup only it
                                if let Some(request_clicks) = description.request_clicks {
                                    info!("Request clicks : {:?}", request_clicks);
                                    self.request_clicks = Some(request_clicks);
                                    self.current_description = None;
                                    self.current_description_state = None;
                                } else {
                                    // If this is a zone reload request
                                    if description.reload_zone {
                                        return vec![MainMessage::SetLoadZoneEngineWithClient(
                                            self.client.clone(),
                                            self.state.player.id.clone(),
                                        )];
                                    } else if let Some(open_new_tab) = &description.open_new_tab {
                                        println!("Open new tab : {}", open_new_tab);
                                        open_url(open_new_tab);
                                    } else {
                                        if description.reload_inventory {
                                            if self.inventory_state.is_some() {
                                                self.make_open_inventory_request();
                                            }
                                        }
                                        self.current_description =
                                            Some(description::UiDescription::new(
                                                description,
                                                self.graphics.clone(),
                                                self.current_description.clone(),
                                            ));
                                        self.current_description_state =
                                            Some(description::UiDescriptionState::default());
                                    }
                                }
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

        vec![]
    }

    fn user_inputs(&mut self) {
        if self.disable_all_user_input_until > get_time()
            || self.disable_all_user_input
            || self.current_description.is_some()
            || self.inventory.is_some()
            || self.request_clicks.is_some()
            || self.chat_state.is_input_focused()
            || self.chat_state.is_mouse_hover()
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
            self.current_action.is_none() && self.click_begin_in_quick_action.is_none() {
                let position_local = base_util::convert_to_local(Vec2::new(pixels_x, pixels_y));
                self.user_inputs
                    .push(UserInput::MovePlayerBy(position_local * 4.5));
            }
        }
    }

    fn camera(&mut self) -> ((i32, i32), (i32, i32)) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let zoom_multiplier = self.zoom_mode.camera_factor();
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
        let right_offset = if self.chat_state.is_display() {
            let chat_display = ChatDisplay::from_env();
            match chat_display {
                ChatDisplay::Right => chat_display.width() * egui_scale(),
                ChatDisplay::Bottom => 0.,
            }
        } else {
            0.
        };

        let zoom_in = self.zoom_mode == ZoomMode::Double;
        if gui::button::draw_zoom_button(&self.graphics, zoom_in, right_offset) {
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

        if gui::button::draw_run_button(&self.graphics, self.running_mode, right_offset) {
            if base_util::mouse_clicked() {
                self.user_inputs.push(UserInput::SwitchRunningMode);
                self.disable_all_user_input_until = get_time() + 0.25;
            }

            self.disable_all_user_input = true;
        }

        let chat_button_active = if self.chat_state.is_display() {
            true
        } else if self.chat_state.have_unread() {
            self.tick_i % 2 == 0
                || self.tick_i % 3 == 0
                || self.tick_i % 4 == 0
                || self.tick_i % 5 == 0
        } else {
            false
        };
        if gui::button::draw_chat_button(&self.graphics, chat_button_active, right_offset) {
            if base_util::mouse_clicked() {
                self.chat_state.set_display(!self.chat_state.is_display());
                self.chat_state.set_just_opened();
                self.disable_all_user_input_until = get_time() + 0.25;
            }
            self.disable_all_user_input = true;
        }
    }

    fn manage_fresh_socket(&mut self) -> (bool, Vec<message::MainMessage>) {
        let mut messages = vec![];

        if self.socket_is_new {
            // Socket just connected
            if web_socket(&self.state).connected() {
                self.socket_is_new = false;

                let event = util::require_around_event(&self.state);
                web_socket(&self.state).send_text(&event);

                let event = util::require_resume_text_event();
                web_socket(&self.state).send_text(&event);

                let event = util::request_chat_event();
                web_socket(&self.state).send_text(&event);

                if let Some(spritesheet_filename) = &self.state.player.spritesheet_filename {
                    // TODO : Very weak !!
                    if spritesheet_filename
                        == "character_spritesheet_c2d9ef6953b1abf43b90743e5807cc5a.png"
                    {
                        // Request character spritesheet creation
                        self.description_request = Some(self.client.get_description_request(
                            format!("/character/{}/spritesheet-setup", self.state.player.id),
                            None,
                            None,
                        ));
                    }
                }

                // FIXME !!!! Que si inconnue de graphics

                if let Some(player_spritesheet) = &self.state.player.spritesheet_filename {
                    messages.push(message::MainMessage::LoadCharacterSpritesheet(
                        self.state.player.id.clone(),
                        player_spritesheet.clone(),
                    ))
                }

                for (character_id, character) in &self.state.characters {
                    if let Some(player_spritesheet) = &character.spritesheet_filename {
                        messages.push(message::MainMessage::LoadCharacterSpritesheet(
                            character_id.clone(),
                            player_spritesheet.clone(),
                        ))
                    }
                }

                let new_coordinates = (self.state.player.zone_row_i, self.state.player.zone_col_i);
                self.last_require_around_coordinate = new_coordinates;
                return (false, messages);
            }

            // Indicate to do nothing while socket is not connected
            return (true, messages);
        }
        return (false, messages);
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

    pub fn zone_position_to_screen_position(&self, row_i: f32, col_i: f32) -> Vec2 {
        let zoom_factor = self.zoom_mode.factor();
        let absolute_position = Vec2::new(
            col_i * self.graphics.tile_width,
            row_i * self.graphics.tile_height,
        );
        let offset = Vec2::new(
            self.state.player_display.position.x - (screen_width() / 2.0) / zoom_factor,
            self.state.player_display.position.y - (screen_height() / 2.0) / zoom_factor,
        );
        let relative_position = absolute_position - offset;
        let relative_position = relative_position * zoom_factor;

        relative_position
    }

    fn manage_text_inputs(&mut self) {
        if let Some(request) = &mut self.chat_text_input_request {
            if let Some(value) = request.try_recv() {
                self.chat_state.set_input_value(value);
                self.chat_state.set_surrender_focus();
            }
        }
    }
}

impl Engine for ZoneEngine {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        // wasm web socket connection must be awaited
        let mut messages = vec![];
        let (fresh_socket, messages_) = self.manage_fresh_socket();
        messages.extend(messages_);
        if fresh_socket {
            return messages;
        }

        self.update_tick_i();
        self.update_frame_i();
        self.consume_events();
        self.user_inputs();
        self.update();
        self.manage_text_inputs();
        self.proceed_quick_action_requests();
        messages.extend(self.proceed_description_requests());
        self.proceed_inventory_requests();
        messages.extend(self.recv_events());
        let draw_area = self.camera();

        // Game
        self.scene(draw_area);
        self.camera_animations();
        self.draw_zone_ux();
        let action_clicked = self.draw_current_action();
        self.draw_request_clicks();

        // Ui
        set_default_camera();
        self.ui_animations();
        self.draw_zone_debug(draw_area);
        self.draw_characters_names(draw_area);
        self.disable_all_user_input = false;
        messages.extend(self.draw_left_panel());
        self.draw_resume_items();
        self.draw_user_logs();
        self.draw_quick_actions(action_clicked);
        self.draw_buttons();
        self.draw_helper_text();
        self.draw_inventory();
        self.draw_blinking_icons();
        self.helper_text = None;

        messages.extend(self.ui());
        egui_macroquad::draw();

        if self.display_debug_info {
            draw_text(&self.debug_info.to_string(), 32.0, 32.0, 32.0, YELLOW);
        }

        messages
    }

    fn signal_illustration_loaded(&mut self, _illustration_name: &str) {
        if let Some(current_description) = self.current_description.as_mut() {
            // Faking is_first_frame will permit to rerun .check_init description function
            current_description.is_first_frame = true;
        }
    }

    fn replace_graphics(&mut self, graphics: crate::graphics::Graphics) {
        self.graphics = graphics.clone();
        if let Some(current_description) = self.current_description.as_mut() {
            current_description.graphics = graphics;
        }
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
    SubmitChatInput,
}

#[derive(PartialEq)]
pub enum ZoomMode {
    Normal,
    Double,
    Large,
}

impl ZoomMode {
    pub fn camera_factor(&self) -> f32 {
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

    pub fn factor(&self) -> f32 {
        self.camera_factor() / 2.0
    }
}

pub enum UserEvent {
    // zone_row_i, zone_col_i, post_base_url
    InventoryItemDropped(usize, usize, String),
}
