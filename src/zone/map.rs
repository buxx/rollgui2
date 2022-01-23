use super::ZoneMapTiles;

pub struct ZoneMap {
    pub tiles: ZoneMapTiles,
}

impl ZoneMap {
    pub fn new(tiles: ZoneMapTiles) -> Self {
        Self { tiles }
    }
}
