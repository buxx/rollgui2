use crate::tileset::TileId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldAsCharacter {
    pub rows: Vec<Vec<TileId>>,
}
