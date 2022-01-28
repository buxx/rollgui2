use macroquad::prelude::*;

use crate::tileset;

#[derive(Clone)]
pub struct Graphics {
    pub tileset_texture: Texture2D,
    pub tiles_mapping: tileset::TileMapping,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl Graphics {
    pub fn new(
        tileset_texture: Texture2D,
        tiles_mapping: tileset::TileMapping,
        tile_width: f32,
        tile_height: f32,
    ) -> Self {
        Self {
            tileset_texture,
            tiles_mapping,
            tile_width,
            tile_height,
        }
    }

    pub fn draw_tile_in_camera(
        &self,
        area_width: f32,
        area_height: f32,
        dest_x: f32,
        dest_y: f32,
        foreground_tile_id: &str,
        background_tile_id: Option<&str>,
        tick_i: i16,
        background_params: Option<DrawTextureParams>,
        foreground_params: Option<DrawTextureParams>,
    ) {
        let camera_dest_x = dest_x / area_width;
        // Invert the value because the camera is Y inverted
        let camera_dest_y = -(dest_y / area_height);

        // Draw tile background
        if let Some(background_tile_id_) = background_tile_id {
            let background_source = self
                .tiles_mapping
                .get(background_tile_id_)
                .expect(&format!("Tile id {} is unknown", background_tile_id_));
            let background_source_rect = background_source.to_rect(tick_i);

            let dest_size_x = self.tile_width / area_width;
            let dest_size_y = self.tile_height / area_height;

            let mut background_params = match background_params {
                Some(background_params) => background_params,
                None => DrawTextureParams::default(),
            };
            background_params.source = Some(background_source_rect);
            background_params.dest_size = Some(Vec2::new(dest_size_x, dest_size_y));
            background_params.flip_y = true; // Invert on Y because camera is Y inverted

            draw_texture_ex(
                self.tileset_texture,
                camera_dest_x,
                camera_dest_y,
                WHITE,
                background_params,
            );
        }

        // Draw tile foreground
        let foreground_source = self
            .tiles_mapping
            .get(foreground_tile_id)
            .expect(&format!("Tile id {} is unknown", foreground_tile_id));
        let foreground_source_rect = foreground_source.to_rect(tick_i);

        let dest_size_x = self.tile_width / area_width;
        let dest_size_y = self.tile_height / area_height;

        let mut foreground_params = match foreground_params {
            Some(foreground_params) => foreground_params,
            None => DrawTextureParams::default(),
        };
        foreground_params.source = Some(foreground_source_rect);
        foreground_params.dest_size = Some(Vec2::new(dest_size_x, dest_size_y));
        foreground_params.flip_y = true; // Invert on Y because camera is Y inverted

        draw_texture_ex(
            self.tileset_texture,
            camera_dest_x,
            camera_dest_y,
            WHITE,
            foreground_params,
        );
    }
}
