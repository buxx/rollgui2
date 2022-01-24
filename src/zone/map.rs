use super::ZoneMapTiles;

pub struct ZoneMap {
    pub tiles: ZoneMapTiles,
    pub background_tile_id: String,
    pub width: usize,
    pub height: usize,
    // Computed map width according to tiles width
    pub concrete_width: f32,
    // Computed map width according to tiles height
    pub concrete_height: f32,
}

impl ZoneMap {
    pub fn new(
        tiles: ZoneMapTiles,
        background_tile_id: &str,
        tiles_width: f32,
        tiles_height: f32,
    ) -> Self {
        let map_width_count = tiles
            .iter()
            .map(|row| row.len())
            .max()
            .expect("Unable to determine zone width count") as f32;
        let map_height_count = tiles.len() as f32;
        let concrete_width = map_width_count * tiles_width;
        let concrete_height = map_height_count * tiles_height;

        Self {
            tiles,
            background_tile_id: background_tile_id.to_string(),
            width: map_width_count as usize,
            height: map_height_count as usize,
            concrete_width,
            concrete_height,
        }
    }
}
