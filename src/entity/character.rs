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
    // TODO : still required ?
    pub fn new(
        id: String,
        world_row_i: i32,
        world_col_i: i32,
        zone_row_i: i32,
        zone_col_i: i32,
        avatar_uuid: Option<String>,
        avatar_is_validated: bool,
    ) -> Self {
        Self {
            id,
            world_row_i,
            world_col_i,
            zone_row_i,
            zone_col_i,
            avatar_uuid,
            avatar_is_validated,
        }
    }
}
