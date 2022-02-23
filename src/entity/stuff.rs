use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stuff {
    pub id: i32,
    pub stuff_id: String,
    pub zone_row_i: i32,
    pub zone_col_i: i32,
    pub classes: Vec<String>,
}

impl Stuff {
    pub fn position(&self) -> (i32, i32) {
        (self.zone_row_i, self.zone_col_i)
    }
    pub fn get_classes(&self) -> Vec<String> {
        // TODO perf: compute this at object creation
        let mut classes = vec!["STUFF_GENERIC".to_string()];
        classes.extend(self.classes.clone());
        classes.push(self.stuff_id.clone());
        classes
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StuffApi {
    pub ids: Vec<i32>,
    pub stuff_id: String,
    pub name: String,
    pub infos: String,
    pub under_construction: bool,
    pub classes: Vec<String>,
    pub is_equipment: bool,
    pub count: i32,
    pub drop_base_url: String,
    pub is_heavy: bool,
    pub is_cumbersome: bool,
    pub is_equip: bool,
}
