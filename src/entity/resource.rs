use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub id: String,
    pub zone_row_i: i32,
    pub zone_col_i: i32,
}

impl Resource {
    pub fn position(&self) -> (i32, i32) {
        (self.zone_row_i, self.zone_col_i)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceApi {
    pub id: String,
    pub name: String,
    pub weight: f32,
    pub clutter: f32,
    pub info: String,
    pub classes: Vec<String>,
    pub quantity: f32,
}
