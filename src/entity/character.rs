use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: String,
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub avatar_uuid: Option<String>,
    pub avatar_is_validated: bool,
    // pub velocity: Vec2,
}

impl Character {
    pub fn new(
        id: String,
        zone_row_i: i32,
        zone_col_i: i32,
        avatar_uuid: Option<String>,
        avatar_is_validated: bool,
    ) -> Self {
        Self {
            id,
            zone_row_i,
            zone_col_i,
            avatar_uuid,
            avatar_is_validated,
        }
    }
}
