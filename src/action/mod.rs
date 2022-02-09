use serde::{Deserialize, Serialize};

pub mod quick;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExploitableTile {
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub classes: Vec<String>,
}

pub struct Action {
    pub post_url: String,
    pub cursor_class: Option<String>,
    pub exploitable_tiles: Vec<ExploitableTile>,
    pub all_tiles_at_once: bool,
}
