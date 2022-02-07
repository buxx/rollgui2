use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    pub name: String,
    pub value_is_str: bool,
    pub value_is_float: bool,
    pub value_str: Option<String>,
    pub value_float: Option<f32>,
    pub url: Option<String>,
    pub classes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListOfItemModel {
    pub items: Vec<ItemModel>,
}
