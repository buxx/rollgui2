use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransportMode {
    #[serde(rename = "WALKING")]
    Walking,
}

impl TransportMode {
    pub fn to_string(&self) -> &str {
        match self {
            TransportMode::Walking => "WALKING",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HumpType {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "SLOW")]
    Slow,
    #[serde(rename = "VERY_SLOW")]
    VerySlow,
}

impl HumpType {
    pub fn to_speed(&self) -> f32 {
        match self {
            HumpType::Normal => 1.0,
            HumpType::Slow => 0.75,
            HumpType::VerySlow => 0.5,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub id: String,
    pub char: char,
    pub traversable: HashMap<TransportMode, bool>,
    pub hump: HashMap<TransportMode, HumpType>,
}
