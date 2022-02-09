use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct QuickAction {
    pub name: String,
    pub base_url: String,
    pub classes: Vec<String>,
    pub exploitable_tiles: Vec<super::ExploitableTile>,
    pub all_tiles_at_once: bool,
}
