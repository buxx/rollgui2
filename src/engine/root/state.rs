use macroquad::prelude::*;

use crate::util::get_remember_me;

pub struct RootState {
    pub first_frame: bool,
    pub login: String,
    pub password: String,
    pub error_message: Option<String>,
    pub loading: bool,
    pub home_message: Option<(String, egui::Color32)>,
    pub root_illustration: Option<egui::TextureHandle>,
    pub remember_me: bool,
}

impl RootState {
    pub fn new() -> Self {
        Self {
            first_frame: true,
            login: "".to_string(),
            password: "".to_string(),
            error_message: None,
            loading: false,
            home_message: None,
            root_illustration: None,
            remember_me: get_remember_me(),
        }
    }
}
