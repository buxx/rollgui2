use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: String,
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub avatar_uuid: Option<String>,
    pub avatar_is_validated: bool,
    pub display_x: f32,
    pub display_y: f32,
}

impl Character {
    pub fn new(
        id: String,
        zone_row_i: i32,
        zone_col_i: i32,
        avatar_uuid: Option<String>,
        avatar_is_validated: bool,
        display_x: f32,
        display_y: f32,
    ) -> Self {
        Self {
            id,
            zone_row_i,
            zone_col_i,
            avatar_uuid,
            avatar_is_validated,
            display_x,
            display_y,
        }
    }
}
