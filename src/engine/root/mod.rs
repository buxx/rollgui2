use crate::{
    client::{self, Client},
    engine::root::util::auth_failed,
    message,
    ui::utils::is_mobile,
    Opt,
};
use macroquad::prelude::*;
use quad_net::http_request::Request;

use super::Engine;
use crate::ui as base_ui;

pub mod state;
pub mod ui;
pub mod util;

pub struct RootScene {
    state: state::RootState,
    do_login_request: Option<Request>,
    text_input_request: Option<base_ui::text_input::TextInputRequest>,
    get_illustrations_names_request: Option<Request>,
    illustrations_names_to_load: Option<Vec<String>>,
}

impl RootScene {
    pub fn new(opt: &Opt) -> Self {
        let get_illustrations_names_request =
            Some(client::Client::get_anonymous_illustrations_names_request());
        Self {
            state: state::RootState::new(
                &opt.login.as_ref().unwrap_or(&"".to_string()),
                &opt.password.as_ref().unwrap_or(&"".to_string()),
                opt.login.is_some() && opt.password.is_some(),
            ),
            do_login_request: None,
            text_input_request: None,
            get_illustrations_names_request,
            illustrations_names_to_load: None,
        }
    }

    pub fn with_home_message(message: String, color: Option<egui::Color32>) -> Self {
        let mut state = state::RootState::new("", "", false);
        state.home_message = Some((message, color.unwrap_or(egui::Color32::WHITE)));

        Self {
            state,
            do_login_request: None,
            text_input_request: None,
            get_illustrations_names_request: None,
            illustrations_names_to_load: None,
        }
    }

    fn manage_do_login(&mut self) -> Vec<RootEvent> {
        let mut events = vec![];

        if let Some(do_login_request) = self.do_login_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match auth_failed(&data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            let character_id = &data.unwrap_or("".to_string());
                            if character_id == "" {
                                events.push(RootEvent::GoToCreateCharacter(
                                    self.state.login.clone(),
                                    self.state.password.clone(),
                                ));
                            } else {
                                events.push(RootEvent::GoToZone(
                                    self.state.login.clone(),
                                    self.state.password.clone(),
                                    character_id.clone(),
                                ));
                            }
                        }
                    }
                    Err(error) => {
                        self.state.error_message = Some(format!("Erreur : {}", error));
                    }
                }

                self.state.loading = false;
                self.do_login_request = None;
            }
        }

        events
    }

    fn manage_text_inputs(&mut self) -> Vec<RootEvent> {
        let mut events = vec![];

        if let Some(request) = &mut self.text_input_request {
            if let Some(value) = request.try_recv() {
                if let Some(text_input) = RootTextInput::from_str(request.name()) {
                    debug!("Text input : {:?} {:?}", &text_input, value);
                    match text_input {
                        RootTextInput::Login => events.push(RootEvent::UpdateLoginValue(value)),
                        RootTextInput::Password => {
                            events.push(RootEvent::UpdatePasswordValue(value))
                        }
                    }
                } else {
                    error!("Unknown text input {}", request.name());
                }
                events.push(RootEvent::RemoveTextInputRequest)
            }
        }

        events
    }
}

impl Engine for RootScene {
    fn tick(&mut self) -> Vec<message::MainMessage> {
        let mut messages = vec![];
        let mut events = vec![];

        // Load illustrations is priority
        if let Some(request) = self.get_illustrations_names_request.as_mut() {
            if let Some(data) = request.try_recv() {
                match data {
                    Ok(illustrations_names_str) => {
                        match serde_json::from_str(&illustrations_names_str) {
                            Ok(illustrations_names) => {
                                self.illustrations_names_to_load = Some(illustrations_names);
                            }
                            Err(error) => {
                                return vec![message::MainMessage::SetErrorEngine(
                                    error.to_string(),
                                )];
                            }
                        };
                        self.get_illustrations_names_request = None;
                    }
                    Err(error) => {
                        return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                    }
                }
            }

            // Display new egui frame with home message
            ui::ui(&mut self.state, true);
            egui_macroquad::draw();

            // Freeze ui during loading
            return vec![];
        }

        // If still illustrations to load
        if let Some(illustrations_names_to_load) = &mut self.illustrations_names_to_load {
            if let Some(illustrations_name_to_load) = illustrations_names_to_load.pop() {
                // Change home message to indicate loading
                self.state.home_message = Some((
                    format!(
                        "Chargement de l'illustration {}",
                        illustrations_name_to_load
                    ),
                    egui::Color32::WHITE,
                ));

                // Display new egui frame with home message
                ui::ui(&mut self.state, true);
                egui_macroquad::draw();

                // Require texture load from main loop
                return vec![message::MainMessage::LoadIllustration(
                    illustrations_name_to_load,
                )];
            } else {
                self.illustrations_names_to_load = None;
                self.state.home_message = None;
            }
        }

        // Accept Enter key for login form
        if is_key_released(KeyCode::Enter) | is_key_released(KeyCode::KpEnter)
            && self.do_login_request.is_none()
        {
            events.push(RootEvent::DoLogin);
        }

        events.extend(self.manage_do_login());
        events.extend(self.manage_text_inputs());
        events.extend(ui::ui(&mut self.state, false));

        for event in events {
            match event {
                RootEvent::QuitGame => {
                    messages.push(message::MainMessage::Quit);
                }
                RootEvent::GoToZone(login, password, character_id) => {
                    messages.push(message::MainMessage::SetLoadZoneEngine(
                        login,
                        password,
                        character_id,
                    ));
                }
                RootEvent::GoToCreateCharacter(login, password) => {
                    messages.push(message::MainMessage::SetLoadDescriptionEngine(
                        "/_describe/character/create".to_string(),
                        None,
                        None,
                        None,
                        None,
                        Some(client::Client::new(login.clone(), password.clone())),
                    ));
                }
                RootEvent::DoLogin => {
                    let request = client::Client::get_current_character_id_request(
                        &self.state.login,
                        &self.state.password,
                    );
                    self.do_login_request = Some(request);
                    self.state.loading = true;
                }
                RootEvent::GoToCreateAccount => {
                    messages.push(message::MainMessage::SetLoadDescriptionEngine(
                        "/account/create".to_string(),
                        None,
                        None,
                        None,
                        None,
                        None,
                    ))
                }
                RootEvent::TextEditFocused(input) => {
                    let value = match input {
                        RootTextInput::Login => self.state.login.clone(),
                        RootTextInput::Password => "".to_string(),
                    };
                    if is_mobile() {
                        self.text_input_request = Some(base_ui::text_input::TextInputRequest::new(
                            input.to_string(),
                            input.to_string(),
                            value,
                        ))
                    }
                }
                RootEvent::RemoveTextInputRequest => {
                    info!("Remove text input request");
                    self.text_input_request = None;
                }
                RootEvent::UpdateLoginValue(login) => {
                    info!("Update login value");
                    self.state.login = login;
                }
                RootEvent::UpdatePasswordValue(password) => {
                    info!("Update password value");
                    self.state.password = password
                }
            }
        }

        egui_macroquad::draw();
        self.state.first_frame = false;
        messages
    }
}

#[derive(Debug)]
pub enum RootTextInput {
    Login,
    Password,
}

impl RootTextInput {
    pub fn to_string(&self) -> String {
        match self {
            RootTextInput::Login => "Login".to_string(),
            RootTextInput::Password => "Password".to_string(),
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Login" => Some(Self::Login),
            "Password" => Some(Self::Password),
            _ => None,
        }
    }
}

pub enum RootEvent {
    QuitGame,
    GoToCreateCharacter(String, String),
    GoToCreateAccount,
    GoToZone(String, String, String),
    DoLogin,
    TextEditFocused(RootTextInput),
    RemoveTextInputRequest,
    UpdateLoginValue(String),
    UpdatePasswordValue(String),
}
