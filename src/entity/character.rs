use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: String,
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub world_row_i: i32,
    pub world_col_i: i32,
    pub avatar_uuid: Option<String>,
    pub avatar_is_validated: bool,
}

impl Character {
    pub fn minimal(id: String, zone_row_i: i32, zone_col_i: i32) -> Self {
        Self {
            id,
            world_row_i: 0,
            world_col_i: 0,
            zone_row_i,
            zone_col_i,
            avatar_uuid: None,
            avatar_is_validated: false,
        }
    }
}
