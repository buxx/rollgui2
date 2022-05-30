use ahash::AHashMap;

pub mod loader;
pub mod source;

pub type TileId = String;
pub type TileMapping = AHashMap<TileId, source::TileSource>;
