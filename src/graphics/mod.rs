use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::io::Cursor;

use macroquad::prelude::*;

use crate::{hardcoded::get_tiles_list, tileset, types::AvatarUuid};

pub mod utils;

#[derive(Clone)]
pub struct Graphics {
    pub tileset_texture: Texture2D,
    pub tiles_mapping: tileset::TileMapping,
    pub tiles_data: HashMap<String, egui::ImageData>,
    pub tile_width: f32,
    pub tile_height: f32,
    pub avatars: HashMap<AvatarUuid, Texture2D>,
}

impl Graphics {
    pub fn new(
        tileset_texture: Texture2D,
        tile_set_bytes: Vec<u8>,
        tiles_mapping: tileset::TileMapping,
        tile_width: f32,
        tile_height: f32,
    ) -> Self {
        // FIXME manage errors
        let tile_set_image = ImageReader::new(Cursor::new(tile_set_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        // TODO : crop all tiles images and make egui texture with it, then store it
        let mut tiles_data = HashMap::new();
        for (tile_id, row_i, col_i, _) in get_tiles_list() {
            let x = col_i as f32 * tile_width;
            let y = row_i as f32 * tile_height;
            let tile_image =
                tile_set_image.crop_imm(x as u32, y as u32, tile_width as u32, tile_height as u32);
            let tile_bytes = tile_image.as_bytes().to_vec();
            let image_data = egui::ImageData::Color(egui::ColorImage::from_rgba_unmultiplied(
                [tile_width as usize, tile_height as usize],
                &tile_bytes,
            ));
            tiles_data.insert(tile_id.to_string(), image_data);
        }

        Self {
            tileset_texture,
            tiles_mapping,
            tiles_data,
            tile_width,
            tile_height,
            avatars: HashMap::new(),
        }
    }

    pub fn find_tile_id_from_classes(&self, classes: &Vec<String>) -> String {
        for class in classes.iter().rev() {
            if self.tiles_mapping.contains_key(class) {
                return class.clone();
            }
        }
        return "UNKNOWN".to_string();
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

    pub fn draw_tile_highlight(
        &self,
        row_i: usize,
        col_i: usize,
        area_width: f32,
        area_height: f32,
    ) {
        draw_rectangle_lines(
            (col_i as f32 * self.tile_width) / area_width,
            -(row_i as f32 * self.tile_width) / area_height,
            self.tile_width / area_width,
            self.tile_height / area_height,
            2.0 / area_width,
            BLUE,
        );
    }

    pub fn add_avatar_texture(&mut self, avatar_uuid: AvatarUuid, texture: Texture2D) {
        self.avatars.insert(avatar_uuid, texture);
    }
}
