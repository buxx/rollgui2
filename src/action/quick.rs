use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::char_to_key_code;

#[derive(Serialize, Deserialize, Debug)]

pub struct QuickAction {
    pub uuid: String,
    pub name: String,
    pub base_url: String,
    pub classes1: Vec<String>,
    pub classes2: Vec<String>,
    pub exploitable_tiles: Vec<super::ExploitableTile>,
    pub all_tiles_at_once: bool,
    pub direct_action: bool,
    pub quick_action_key: Option<char>,
    pub force_open_description: bool,
}

impl QuickAction {
    pub fn quick_action_key_code(&self) -> Option<KeyCode> {
        if let Some(quick_action_key) = self.quick_action_key {
            if let Some(key_code) = char_to_key_code(&quick_action_key) {
                Some(key_code)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn quick_action_key_pressed(&self) -> bool {
        if let Some(key_code) = self.quick_action_key_code() {
            return is_key_pressed(key_code);
        }

        false
    }
}
