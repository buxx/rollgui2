use macroquad::prelude::*;
use quad_net::http_request::Request;

use crate::{
    client::Client,
    entity::{character::Character, world::WorldAsCharacter},
    graphics::Graphics,
    message::MainMessage,
    ui::utils::is_mobile,
    util::mouse_clicked,
};

use super::Engine;

const BACK_BUTTON_START_X: f32 = 960.0;
const BACK_BUTTON_START_Y: f32 = 704.0;
const BACK_BUTTON_WIDTH: f32 = 64.0;
const BACK_BUTTON_HEIGHT: f32 = 64.0;
const DRAW_BACK_BUTTON_WIDTH: f32 = 64.0;
const DRAW_BACK_BUTTON_HEIGHT: f32 = 64.0;

pub struct WorldEngine {
    graphics: Graphics,
    client: Client,
    player: Character,
    world_as_character_request: Option<Request>,
    world_as_character: Option<WorldAsCharacter>,
    frame_i: i32,
    tick_i: i32,
}

impl WorldEngine {
    pub fn new(graphics: Graphics, client: Client, player: Character) -> WorldEngine {
        let world_as_character_request = Some(client.get_world_as_character_request(&player.id));
        WorldEngine {
            graphics,
            client,
            player,
            world_as_character_request,
            world_as_character: None,
            frame_i: 0,
            tick_i: 0,
        }
    }

    fn manage_world_as_character_request(&mut self) -> Vec<MainMessage> {
        if let Some(request) = self.world_as_character_request.as_mut() {
            if let Some(data) = request.try_recv() {
                info!("World as character received");
                match data {
                    Ok(world_as_character_str) => {
                        let world_as_character: WorldAsCharacter =
                            match serde_json::from_str(&world_as_character_str) {
                                Ok(world_as_character) => world_as_character,
                                Err(error) => {
                                    return vec![MainMessage::SetErrorEngine(error.to_string())]
                                }
                            };

                        self.world_as_character = Some(world_as_character);
                    }
                    Err(error) => {
                        return vec![MainMessage::SetErrorEngine(error.to_string())];
                    }
                }
                self.world_as_character_request = None;
            };
        }

        vec![]
    }

    fn display(&mut self) -> Vec<MainMessage> {
        let mut messages = vec![];

        if self.world_as_character.is_none() {
            messages.extend(self.loading())
        }

        messages.extend(self.world());
        messages.extend(self.back_button());

        messages
    }

    fn loading(&mut self) -> Vec<MainMessage> {
        draw_text(
            "Chargement ...",
            (screen_width() / 2.) - 128.,
            screen_height() / 2.,
            64.,
            WHITE,
        );

        vec![]
    }

    fn world(&mut self) -> Vec<MainMessage> {
        if let Some(world) = &self.world_as_character {
            for (row_i, row) in world.rows.iter().enumerate() {
                let dest_y = row_i as f32 * self.graphics.tile_height;

                for (col_i, tile_id) in row.iter().enumerate() {
                    let dest_x = col_i as f32 * self.graphics.tile_width;

                    let source = self.graphics.tiles_mapping.get(tile_id).unwrap().to_rect(0);
                    draw_texture_ex(
                        self.graphics.tileset_texture,
                        dest_x,
                        dest_y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(source),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // Display character
        if self.tick_i % 2 == 0 {
            let dest_x = self.player.world_col_i as f32 * self.graphics.tile_width;
            let dest_y = self.player.world_row_i as f32 * self.graphics.tile_height;

            draw_texture_ex(
                self.graphics.tileset_texture,
                dest_x,
                dest_y,
                WHITE,
                DrawTextureParams {
                    source: Some(
                        self.graphics
                            .tiles_mapping
                            .get("CHARACTER")
                            .unwrap()
                            .to_rect(0),
                    ),
                    ..Default::default()
                },
            );
        }

        vec![]
    }

    fn back_button(&self) -> Vec<MainMessage> {
        let dest_size = if is_mobile() {
            Vec2::new(DRAW_BACK_BUTTON_WIDTH, DRAW_BACK_BUTTON_HEIGHT)
        } else {
            Vec2::new(DRAW_BACK_BUTTON_WIDTH * 2.0, DRAW_BACK_BUTTON_HEIGHT * 2.0)
        };
        let draw_back_button_to_x: f32 = screen_width() - dest_size.x - 25.0;
        let draw_back_button_to_y: f32 = 25.0;
        draw_texture_ex(
            self.graphics.tileset_texture,
            draw_back_button_to_x,
            draw_back_button_to_y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    BACK_BUTTON_START_X,
                    BACK_BUTTON_START_Y,
                    BACK_BUTTON_WIDTH,
                    BACK_BUTTON_HEIGHT,
                )),
                dest_size: Some(dest_size),
                ..Default::default()
            },
        );

        if mouse_clicked() {
            let mouse_position = mouse_position();
            if mouse_position.0 >= draw_back_button_to_x
                && mouse_position.0 < draw_back_button_to_x + dest_size.x
                && mouse_position.1 >= draw_back_button_to_y
                && mouse_position.1 < draw_back_button_to_y + dest_size.y
            {
                return vec![MainMessage::SetLoadZoneEngineWithClient(
                    self.client.clone(),
                    self.player.id.clone(),
                )];
            }
        }

        vec![]
    }
}

impl Engine for WorldEngine {
    fn tick(&mut self) -> Vec<MainMessage> {
        let mut messages = vec![];

        messages.extend(self.manage_world_as_character_request());
        messages.extend(self.display());

        if is_key_released(KeyCode::Escape) {
            messages.extend([MainMessage::SetLoadZoneEngineWithClient(
                self.client.clone(),
                self.player.id.clone(),
            )]);
        }

        self.frame_i += 1;
        if self.frame_i % 15 == 0 {
            self.tick_i += 1;
        }

        messages
    }

    fn replace_graphics(&mut self, graphics: Graphics) {
        self.graphics = graphics;
    }

    fn signal_illustration_loaded(&mut self, _illustration_name: &str) {}
}
