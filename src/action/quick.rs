use serde::{Deserialize, Serialize};

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
