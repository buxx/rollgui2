use macroquad::prelude::*;

use crate::{graphics, tileset, zone};

pub struct TileDropAnimation {
    source: tileset::source::TileSource,
    camera_dest: Vec2,
    camera_dest_size: Vec2,
}

impl super::Animation for TileDropAnimation {
    fn update(&mut self, _frame_i: i64) -> bool {
        self.camera_dest_size /= 1.1;
        self.camera_dest_size.length() < 0.05
    }

    fn draw_in_camera(&self, graphics: &graphics::Graphics) {
        // TODO : tick_i ?
        let source_rect = self.source.to_rect(0);
        let params = DrawTextureParams {
            dest_size: Some(self.camera_dest_size),
            source: Some(source_rect),
            ..Default::default()
        };
        draw_texture_ex(
            graphics.tileset_texture,
            self.camera_dest.x,
            self.camera_dest.y,
            WHITE,
            params,
        );
    }
}

impl TileDropAnimation {
    pub fn new(
        graphics: &graphics::Graphics,
        map: &zone::map::ZoneMap,
        tile_id: &str,
        row_i: i32,
        col_i: i32,
    ) -> Result<Self, String> {
        let source = match graphics.tiles_mapping.get(tile_id) {
            Some(background_source_) => background_source_.clone(),
            None => return Err(format!("Tile id {} is unknown", tile_id)),
        };

        let dest_x = col_i as f32 * graphics.tile_width;
        let dest_y = row_i as f32 * graphics.tile_height;

        let camera_dest_x = dest_x / map.concrete_width;
        // Invert the value because the camera is Y inverted
        let camera_dest_y = -(dest_y / map.concrete_height);

        let dest_size_x = graphics.tile_width / map.concrete_width;
        let dest_size_y = graphics.tile_height / map.concrete_height;

        Ok(Self {
            source,
            camera_dest: Vec2::new(camera_dest_x, camera_dest_y),
            camera_dest_size: Vec2::new(dest_size_x, dest_size_y) * 60.0 * 1.1,
        })
    }
}
