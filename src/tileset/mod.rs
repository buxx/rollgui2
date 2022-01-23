use std::collections::HashMap;

pub mod loader;
pub mod source;

pub type TileId = String;
pub type TileMapping = HashMap<TileId, source::TileSource>;
