use serde::{Deserialize, Serialize};

pub mod quick;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExploitableTile {
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub classes: Vec<String>,
}

pub struct Action {
    pub uuid: String,
    pub post_url: String,
    pub cursor_class: Option<String>,
    pub exploitable_tiles: Vec<ExploitableTile>,
    pub all_tiles_at_once: bool,
}

impl Action {
    pub fn from_quick_action(quick_action: &quick::QuickAction) -> Action {
        Self {
            uuid: quick_action.uuid.clone(),
            post_url: quick_action.base_url.clone(),
            cursor_class: None,
            exploitable_tiles: quick_action.exploitable_tiles.clone(),
            all_tiles_at_once: quick_action.all_tiles_at_once,
        }
    }
}
