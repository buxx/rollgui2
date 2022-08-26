use crate::{
    client::{self, Client},
    engine::root::util::auth_failed,
    graphics::Graphics,
    message,
    ui::utils::is_mobile,
    util::{get_auth_token, get_remember_me, set_auth_token, set_remember_me},
};
use macroquad::prelude::*;
use quad_net::http_request::Request;

use super::Engine;
use crate::ui as base_ui;

pub mod state;
pub mod ui;
pub mod util;

pub struct RootScene {
    graphics: Graphics,
    state: state::RootState,
    do_get_auth_token_request: Option<Request>,
    do_login_with_credentials_request: Option<Request>,
    do_login_with_auth_token_request: Option<Request>,
    text_input_request: Option<base_ui::text_input::TextInputRequest>,
}

impl RootScene {
    pub fn new(graphics: Graphics) -> Self {
        Self {
            graphics,
            state: state::RootState::new(),
            do_get_auth_token_request: None,
            do_login_with_credentials_request: None,
            do_login_with_auth_token_request: None,
            text_input_request: None,
        }
    }

    pub fn with_home_message(
        message: String,
        color: Option<egui::Color32>,
        graphics: Graphics,
    ) -> Self {
        let mut state = state::RootState::new();
        state.home_message = Some((message, color.unwrap_or(egui::Color32::WHITE)));

        Self {
            graphics,
            state,
            do_login_with_credentials_request: None,
            do_get_auth_token_request: None,
            do_login_with_auth_token_request: None,
            text_input_request: None,
        }
    }

    fn manage_do_login(&mut self) -> Vec<RootEvent> {
        let mut events = vec![];

        if let Some(do_auth_token_request) = self.do_get_auth_token_request.as_mut() {
            if let Some(data) = do_auth_token_request.try_recv() {
                match auth_failed(&data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            let auth_token = &data.unwrap_or("".to_string());
                            set_auth_token(Some(auth_token));
                            self.do_login_with_credentials_request = Some(
                                client::Client::with_auth_token(auth_token.clone())
                                    .get_current_character_id_request(),
                            );
                        }
                    }
                    Err(error) => {
                        self.state.error_message = Some(format!("Erreur : {}", error));
                    }
                }
                self.do_get_auth_token_request = None;
            }
        }

        if let Some(do_login_request) = self.do_login_with_credentials_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match auth_failed(&data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            let character_id = &data.unwrap_or("".to_string());
                            if character_id == "" {
                                events.push(RootEvent::GoToCreateCharacter(
                                    client::Client::with_credentials(
                                        self.state.login.clone(),
                                        self.state.password.clone(),
                                    ),
                                ));
                            } else {
                                events.push(RootEvent::GoToZone(
                                    client::Client::with_credentials(
                                        self.state.login.clone(),
                                        self.state.password.clone(),
                                    ),
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
                self.do_login_with_credentials_request = None;
            }
        }

        if let Some(do_login_request) = self.do_login_with_auth_token_request.as_mut() {
            if let Some(data) = do_login_request.try_recv() {
                match auth_failed(&data) {
                    Ok(auth_failed_) => {
                        if auth_failed_ {
                            set_auth_token(None);
                            self.state.error_message = Some("Authentification échoué".to_string());
                        } else {
                            let character_id = &data.unwrap_or("".to_string());
                            let auth_token = get_auth_token()
                                .expect("Auth token must be defined after login with it");
                            if character_id == "" {
                                events.push(RootEvent::GoToCreateCharacter(
                                    client::Client::with_auth_token(auth_token.clone()),
                                ));
                            } else {
                                events.push(RootEvent::GoToZone(
                                    client::Client::with_auth_token(auth_token.clone()),
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
                self.do_login_with_auth_token_request = None;
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

        // Accept Enter key for login form
        if is_key_released(KeyCode::Enter) | is_key_released(KeyCode::KpEnter)
            && self.do_login_with_credentials_request.is_none()
        {
            events.push(RootEvent::DoLoginWithCredentials);
        }

        // Do login with auth if set
        if self.state.first_frame
            && get_remember_me()
            && get_auth_token().is_some()
            && self.do_login_with_auth_token_request.is_none()
        {
            events.push(RootEvent::DoLoginWithAuthToken);
        }

        events.extend(self.manage_do_login());
        events.extend(self.manage_text_inputs());
        events.extend(ui::ui(&mut self.state, &self.graphics));

        for event in events {
            match event {
                RootEvent::QuitGame => {
                    messages.push(message::MainMessage::Quit);
                }
                RootEvent::GoToZone(client, character_id) => {
                    messages.push(message::MainMessage::SetLoadZoneEngine(
                        client,
                        character_id,
                    ));
                }
                RootEvent::GoToCreateCharacter(client) => {
                    messages.push(message::MainMessage::SetLoadDescriptionEngine(
                        "/_describe/character/create".to_string(),
                        None,
                        None,
                        None,
                        None,
                        Some(client),
                    ));
                }
                RootEvent::GoToPasswordLost => {
                    messages.push(message::MainMessage::SetLoadDescriptionEngine(
                        "/account/password_lost".to_string(),
                        None,
                        None,
                        None,
                        None,
                        None,
                    ));
                }
                RootEvent::DoLoginWithCredentials => {
                    // Remember me or not, delete knew auth token : A new auth token
                    // will be set if needed
                    set_auth_token(None);

                    if self.state.remember_me {
                        // Claim a new auth token, login will be made after it
                        self.do_get_auth_token_request =
                            Some(client::Client::get_auth_token_request(
                                &self.state.login,
                                &self.state.password,
                            ));
                    } else {
                        // Directly login with credentials
                        self.do_login_with_credentials_request = Some(
                            client::Client::with_credentials(
                                self.state.login.clone(),
                                self.state.password.clone(),
                            )
                            .get_current_character_id_request(),
                        );
                    }
                    set_remember_me(self.state.remember_me);
                    self.state.loading = true;
                }
                RootEvent::DoLoginWithAuthToken => {
                    if let Some(auth_token) = get_auth_token() {
                        self.do_login_with_auth_token_request = Some(
                            Client::with_auth_token(auth_token).get_current_character_id_request(),
                        );
                        self.state.loading = true;
                    } else {
                        error!("Can't login with auth token without stored auth token");
                    }
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

    fn signal_illustration_loaded(&mut self, _illustration_name: &str) {}

    fn replace_graphics(&mut self, _graphics: crate::graphics::Graphics) {}
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
    GoToPasswordLost,
    GoToCreateCharacter(Client),
    GoToCreateAccount,
    GoToZone(Client, String),
    DoLoginWithCredentials,
    DoLoginWithAuthToken,
    TextEditFocused(RootTextInput),
    RemoveTextInputRequest,
    UpdateLoginValue(String),
    UpdatePasswordValue(String),
}
