use image::{io::Reader as ImageReader, DynamicImage};
use std::collections::HashMap;
use std::io::Cursor;

use macroquad::prelude::*;

use crate::engine::zone::PlayerRunning;
use crate::util::bytes_from_cache_or_file;
use crate::{hardcoded::get_tiles_list, tileset, types::AvatarUuid};

const NUMBER_START_X: f32 = 0.;
const NUMBER_START_Y: f32 = 704.;
const NUMBER_WIDTH: f32 = 32.;
const NUMBER_HEIGHT: f32 = 32.;

enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    fn x(&self) -> f32 {
        match self {
            Number::Zero => NUMBER_START_X + NUMBER_WIDTH * 0.,
            Number::One => NUMBER_START_X + NUMBER_WIDTH * 1.,
            Number::Two => NUMBER_START_X + NUMBER_WIDTH * 2.,
            Number::Three => NUMBER_START_X + NUMBER_WIDTH * 3.,
            Number::Four => NUMBER_START_X + NUMBER_WIDTH * 4.,
            Number::Five => NUMBER_START_X + NUMBER_WIDTH * 5.,
            Number::Six => NUMBER_START_X + NUMBER_WIDTH * 6.,
            Number::Seven => NUMBER_START_X + NUMBER_WIDTH * 7.,
            Number::Eight => NUMBER_START_X + NUMBER_WIDTH * 8.,
            Number::Nine => NUMBER_START_X + NUMBER_WIDTH * 9.,
        }
    }

    fn from_digit(digit: u8) -> Self {
        match digit {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => panic!("Invalid digit"),
        }
    }

    fn from_number(number: u32) -> (Option<Self>, Self) {
        if number < 10 {
            return (None, Self::from_digit(number as u8));
        }

        if number < 99 {
            let tens = (number / 10) as u8;
            let ones = (number % 10) as u8;
            return (Some(Self::from_digit(tens)), Self::from_digit(ones));
        }

        (Some(Self::from_digit(9)), Self::from_digit(9))
    }
}

#[derive(Clone)]
pub struct Graphics {
    pub tile_set_image: DynamicImage,
    pub tileset_texture: Texture2D,
    pub tiles_mapping: tileset::TileMapping,
    pub tiles_bytes: HashMap<String, Vec<u8>>,
    pub tiles_data: HashMap<String, egui::ImageData>,
    pub tile_width: f32,
    pub tile_height: f32,
    pub avatars: HashMap<AvatarUuid, Texture2D>,
    pub illustrations: HashMap<String, egui::ImageData>,
    pub character_spritesheets: HashMap<String, (String, Texture2D)>,
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
        let mut tiles_bytes = HashMap::new();
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
            tiles_bytes.insert(tile_id.to_string(), tile_bytes);
            tiles_data.insert(tile_id.to_string(), image_data);
        }

        Self {
            tile_set_image,
            tileset_texture,
            tiles_mapping,
            tiles_bytes,
            tiles_data,
            tile_width,
            tile_height,
            avatars: HashMap::new(),
            illustrations: HashMap::new(),
            character_spritesheets: HashMap::new(),
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

    pub fn tile_with_ap(&self, tile_id: &str, cost: f32) -> Option<egui::ImageData> {
        if let Some(sprite) = self.tiles_mapping.get(tile_id) {
            let (tens, ones) = Number::from_number(cost as u32);

            let ones_image = self
                .tile_set_image
                .crop_imm(
                    ones.x() as u32,
                    NUMBER_START_Y as u32,
                    NUMBER_WIDTH as u32,
                    NUMBER_HEIGHT as u32,
                )
                .resize(
                    (NUMBER_WIDTH / 2.) as u32,
                    (NUMBER_HEIGHT / 2.0) as u32,
                    image::imageops::FilterType::Nearest,
                );

            let mut final_image = self.tile_set_image.crop_imm(
                sprite.sprites[0].x as u32,
                sprite.sprites[0].y as u32,
                sprite.width as u32,
                sprite.height as u32,
            );
            image::imageops::overlay(
                &mut final_image,
                &ones_image,
                (sprite.width / 2.0) as i64,
                (sprite.height / 2.0) as i64,
            );

            if let Some(tens) = tens {
                let tens_image = self
                    .tile_set_image
                    .crop_imm(
                        tens.x() as u32,
                        NUMBER_START_Y as u32,
                        NUMBER_WIDTH as u32,
                        NUMBER_HEIGHT as u32,
                    )
                    .resize(
                        (NUMBER_WIDTH / 2.) as u32,
                        (NUMBER_HEIGHT / 2.0) as u32,
                        image::imageops::FilterType::Nearest,
                    );

                image::imageops::overlay(
                    &mut final_image,
                    &tens_image,
                    0,
                    (sprite.height / 2.0) as i64,
                );
            }

            return Some(egui::ImageData::Color(
                egui::ColorImage::from_rgba_unmultiplied(
                    [self.tile_width as usize, self.tile_height as usize],
                    &final_image.as_bytes().to_vec(),
                ),
            ));
        }

        None
    }

    pub fn draw_character_in_camera(
        &self,
        area_width: f32,
        area_height: f32,
        dest_x: f32,
        dest_y: f32,
        tick_i: i16,
        character_id: &str,
        running: &Option<PlayerRunning>,
    ) {
        let (x, y) = match running {
            Some(PlayerRunning::Top) => (0. + (64. * tick_i as f32), 512.),
            Some(PlayerRunning::Down) => (0. + (64. * tick_i as f32), 640.),
            Some(PlayerRunning::Left) => (0. + (64. * tick_i as f32), 576.),
            Some(PlayerRunning::Right) => (0. + (64. * tick_i as f32), 704.),
            None => (0., 640.),
        };
        let source = Rect::new(x, y, 64., 64.);
        if let Some((_, texture)) = self.character_spritesheets.get(character_id) {
            let camera_dest_x = dest_x / area_width;
            // Invert the value because the camera is Y inverted
            let camera_dest_y = -(dest_y / area_height);
            let dest_size_x = (self.tile_width / area_width) * 2.;
            let dest_size_y = (self.tile_height / area_height) * 2.;

            draw_texture_ex(
                *texture,
                camera_dest_x - (dest_size_x / 3.7),
                camera_dest_y + dest_size_y,
                WHITE,
                DrawTextureParams {
                    source: Some(source),
                    dest_size: Some(Vec2::new(dest_size_x, -dest_size_y)),
                    ..Default::default()
                },
            );
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

    pub fn add_character_spritesheet(
        &mut self,
        character_id: &str,
        filename: &str,
        texture: Texture2D,
    ) {
        dbg!(self.character_spritesheets.get(character_id));
        self.character_spritesheets
            .insert(character_id.to_string(), (filename.to_string(), texture));
        dbg!(self.character_spritesheets.get(character_id));
    }

    pub async fn load_illustration(&mut self, illustration_name: &str) {
        match bytes_from_cache_or_file(&format!("media/{}", illustration_name), true).await {
            Ok(illustration_bytes) => {
                // TODO : used to determine image size, but some heavy no ?
                let illustration_image = ImageReader::new(Cursor::new(illustration_bytes.clone()))
                    .with_guessed_format()
                    .unwrap()
                    .decode()
                    .unwrap();

                let illustration_data =
                    egui::ImageData::Color(egui::ColorImage::from_rgba_unmultiplied(
                        [
                            illustration_image.width() as usize,
                            illustration_image.height() as usize,
                        ],
                        &illustration_image.to_rgba8(),
                    ));
                self.illustrations
                    .insert(illustration_name.to_string(), illustration_data);
            }
            Err(error) => {
                error!("Error during illustration loading : {}", error);
            }
        };
    }
}
