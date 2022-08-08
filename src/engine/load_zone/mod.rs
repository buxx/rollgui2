use macroquad::prelude::*;
use quad_net::http_request::Request;
use serde_json::Value;

use crate::{
    client, engine::dead::CheckCharacterIsDeadEngine, entity, hardcoded, message,
    types::AvatarUuid, zone,
};

use super::Engine;

pub struct LoadZoneEngine {
    graphics: crate::graphics::Graphics,
    client: client::Client,
    character_id: String,
    get_player_request: Option<Request>,
    player: Option<entity::character::Character>,
    get_tiles_request: Option<Request>,
    tiles: Option<Vec<entity::tile::Tile>>,
    get_zone_request: Option<Request>,
    zone: Option<zone::map::ZoneMap>,
    get_characters_request: Option<Request>,
    characters: Option<Vec<entity::character::Character>>,
    get_player_avatar_request: Option<Request>,
    player_avatar: Option<Vec<u8>>,
    get_avatars_zone_thumbs_requests: Vec<(AvatarUuid, Request)>,
    avatars_zone_thumbs: Vec<(AvatarUuid, Vec<u8>)>,
    get_resources_request: Option<Request>,
    resources: Option<Vec<entity::resource::Resource>>,
    get_stuffs_request: Option<Request>,
    stuffs: Option<Vec<entity::stuff::Stuff>>,
    get_builds_request: Option<Request>,
    builds: Option<Vec<entity::build::Build>>,
}

impl LoadZoneEngine {
    pub fn new(
        graphics: crate::graphics::Graphics,
        login: &str,
        password: &str,
        character_id: &str,
    ) -> Result<Self, String> {
        let client = client::Client::new(login.to_string(), password.to_string());
        Ok(Self {
            graphics,
            client,
            character_id: character_id.to_string(),
            get_player_request: None,
            player: None,
            get_tiles_request: None,
            tiles: None,
            get_zone_request: None,
            zone: None,
            get_characters_request: None,
            characters: None,
            get_player_avatar_request: None,
            player_avatar: None,
            get_avatars_zone_thumbs_requests: vec![],
            avatars_zone_thumbs: vec![],
            get_resources_request: None,
            resources: None,
            get_stuffs_request: None,
            stuffs: None,
            get_builds_request: None,
            builds: None,
        })
    }

    fn make_player_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_player_request.is_none() && self.player.is_none() {
            info!("Request player character");
            self.get_player_request = Some(self.client.get_character_request(&self.character_id));
        }

        vec![]
    }

    fn make_player_avatar_request(&mut self) -> Vec<message::MainMessage> {
        if let Some(player) = &self.player {
            if self.get_player_avatar_request.is_none() && self.player_avatar.is_none() {
                info!("Request player avatar");
                let avatar_uuid = player.private_avatar_uuid();
                let storage = &mut quad_storage::STORAGE.lock().unwrap();
                if let Some(avatar_as_b64) =
                    storage.get(&format!("player_avatar__{}", avatar_uuid.0))
                {
                    // TODO : manage unwrap
                    let avatar_bytes = base64::decode(&avatar_as_b64).unwrap();
                    self.player_avatar = Some(avatar_bytes);
                } else {
                    let request = self.client.get_avatar_request(&avatar_uuid);
                    self.get_player_avatar_request = Some(request);
                };
            }
        }

        vec![]
    }

    fn retrieve_player(&mut self) -> Vec<message::MainMessage> {
        if let Some(get_player_request) = self.get_player_request.as_mut() {
            if let Some(data) = get_player_request.try_recv() {
                info!("Player character received");
                match data {
                    Ok(character_json_str) => {
                        let character: entity::character::Character =
                            match serde_json::from_str(&character_json_str) {
                                Ok(character) => character,
                                Err(_error) => {
                                    // In case of error, maybe the character is dead

                                    return vec![message::MainMessage::SetEngine(Box::new(
                                        CheckCharacterIsDeadEngine::new(
                                            self.character_id.clone(),
                                            self.client.clone(),
                                        ),
                                    ))];
                                }
                            };
                        self.player = Some(character);
                        debug!("{:?}", self.player);
                    }
                    Err(error) => {
                        return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                    }
                }
            }
        };

        vec![]
    }

    fn retrieve_player_avatar(&mut self) -> Vec<message::MainMessage> {
        if let Some(get_player_avatar_request) = self.get_player_avatar_request.as_mut() {
            if let Some(data) = get_player_avatar_request.try_recv() {
                let player = self
                    .player
                    .as_ref()
                    .expect("Player must be defined at this point");
                let avatar_uuid = player.private_avatar_uuid();
                info!("Player avatar received");
                match data {
                    Ok(avatar) => {
                        let avatar_ = avatar.as_bytes();
                        self.player_avatar = Some(avatar_.to_vec());
                        // Store in cache too
                        let storage = &mut quad_storage::STORAGE.lock().unwrap();
                        let avatar_as_b64 = base64::encode(&avatar_);
                        storage.set(&format!("player_avatar__{}", avatar_uuid), &avatar_as_b64);
                    }
                    Err(error) => {
                        return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                    }
                }
            }
        };

        vec![]
    }

    fn make_tiles_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_tiles_request.is_none() {
            info!("Request tiles");
            self.get_tiles_request = Some(self.client.get_tiles_request());
        }

        vec![]
    }

    fn make_zone_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_zone_request.is_none() {
            if let Some(player) = &self.player {
                info!("Request zone");
                self.get_zone_request = Some(
                    self.client
                        .get_zone_request(player.world_row_i, player.world_col_i),
                );
            }
        }

        vec![]
    }

    fn make_characters_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_characters_request.is_none() {
            if let Some(player) = &self.player {
                info!("Request characters");
                self.get_characters_request = Some(
                    self.client
                        .get_characters_request(player.world_row_i, player.world_col_i),
                );
            }
        }

        vec![]
    }

    fn make_avatars_zone_thumbs_requests(&mut self) -> Vec<message::MainMessage> {
        if let Some(characters) = &self.characters {
            for character in characters {
                // Grab avatar from local storage if it exists or make a request to get it
                let avatar_uuid = character.public_avatar_uuid();
                let storage = &mut quad_storage::STORAGE.lock().unwrap();
                if let Some(avatar_as_b64) =
                    storage.get(&format!("avatar_zone_thumb__{}", avatar_uuid.0))
                {
                    // TODO : manage unwrap
                    let avatar_bytes = base64::decode(&avatar_as_b64).unwrap();
                    self.avatars_zone_thumbs
                        .push((avatar_uuid.clone(), avatar_bytes));
                } else {
                    let request = self.client.get_avatar_zone_thumb_request(&avatar_uuid);
                    self.get_avatars_zone_thumbs_requests
                        .push((avatar_uuid.clone(), request));
                };
            }
        };

        vec![]
    }

    fn make_resources_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_resources_request.is_none() {
            if let Some(player) = &self.player {
                info!("Request resources");
                self.get_resources_request = Some(
                    self.client
                        .get_resources_request(player.world_row_i, player.world_col_i),
                );
            }
        }

        vec![]
    }

    fn make_stuffs_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_stuffs_request.is_none() {
            if let Some(player) = &self.player {
                info!("Request stuff");
                self.get_stuffs_request = Some(
                    self.client
                        .get_stuffs_request(player.world_row_i, player.world_col_i),
                );
            }
        }

        vec![]
    }

    fn make_builds_request(&mut self) -> Vec<message::MainMessage> {
        if self.get_builds_request.is_none() {
            if let Some(player) = &self.player {
                info!("Request stuff");
                self.get_builds_request = Some(
                    self.client
                        .get_builds_request(player.world_row_i, player.world_col_i),
                );
            }
        }

        vec![]
    }

    fn retrieve_tiles(&mut self) -> Vec<message::MainMessage> {
        if self.tiles.is_none() {
            if let Some(get_tiles_request) = self.get_tiles_request.as_mut() {
                if let Some(data) = get_tiles_request.try_recv() {
                    info!("Tiles received");
                    match data {
                        Ok(tiles_str) => {
                            let tiles: Vec<entity::tile::Tile> =
                                match serde_json::from_str(&tiles_str) {
                                    Ok(tiles) => tiles,
                                    Err(error) => {
                                        return vec![message::MainMessage::SetErrorEngine(
                                            error.to_string(),
                                        )]
                                    }
                                };

                            self.tiles = Some(tiles);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }

    fn retrieve_zone(&mut self) -> Vec<message::MainMessage> {
        if self.zone.is_none() {
            if let (Some(get_zone_request), Some(tiles)) =
                (self.get_zone_request.as_mut(), &self.tiles)
            {
                if let Some(data) = get_zone_request.try_recv() {
                    info!("Zone received");
                    match data {
                        Ok(zone_str) => {
                            let source_value: Value = serde_json::from_str(&zone_str).unwrap();
                            let source = source_value["raw_source"].as_str().unwrap();
                            let zone_type_id = source_value["zone_type_id"].as_str().unwrap();
                            let default_tile_id =
                                hardcoded::get_default_tile_id_for_zone_type_id(zone_type_id)
                                    .expect(&format!("Unknown world type id {}", zone_type_id));
                            let map: zone::map::ZoneMap = match zone::load::from_txt_map(
                                source,
                                tiles.clone(),
                                self.graphics.tile_width,
                                self.graphics.tile_height,
                                &default_tile_id,
                            ) {
                                Ok(map) => map,
                                Err(error) => {
                                    return vec![message::MainMessage::SetErrorEngine(
                                        error.to_string(),
                                    )]
                                }
                            };
                            self.zone = Some(map);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }

    fn retrieve_characters(&mut self) -> Vec<message::MainMessage> {
        if self.characters.is_none() {
            if let Some(get_characters_request) = self.get_characters_request.as_mut() {
                if let Some(data) = get_characters_request.try_recv() {
                    info!("Characters received");
                    match data {
                        Ok(characters_json_str) => {
                            let characters: Vec<entity::character::Character> =
                                match serde_json::from_str(&characters_json_str) {
                                    Ok(characters) => characters,
                                    Err(error) => {
                                        return vec![message::MainMessage::SetErrorEngine(
                                            error.to_string(),
                                        )]
                                    }
                                };
                            self.characters = Some(characters);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }

    fn retrieve_avatars_zone_thumbs(&mut self) -> Vec<message::MainMessage> {
        for (avatar_uuid, get_avatar_zone_thumb_request) in
            &mut self.get_avatars_zone_thumbs_requests
        {
            if let Some(data) = get_avatar_zone_thumb_request.try_recv() {
                info!("Avatar '{}' zone thumbs received", avatar_uuid);
                match data {
                    Ok(avatar) => {
                        let avatar_ = avatar.as_bytes();
                        self.avatars_zone_thumbs
                            .push((avatar_uuid.clone(), avatar_.to_vec()));
                        // Put this image in cache too
                        let storage = &mut quad_storage::STORAGE.lock().unwrap();
                        storage.set(
                            &format!("avatar_zone_thumb__{}", avatar_uuid),
                            &base64::encode(avatar_),
                        );
                    }
                    Err(error) => {
                        return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                    }
                }
            }
        }

        vec![]
    }

    fn retrieve_resources(&mut self) -> Vec<message::MainMessage> {
        if self.resources.is_none() {
            if let Some(get_resources_request) = self.get_resources_request.as_mut() {
                if let Some(data) = get_resources_request.try_recv() {
                    info!("Resources received");
                    match data {
                        Ok(resources_json_str) => {
                            let resources: Vec<entity::resource::Resource> =
                                match serde_json::from_str(&resources_json_str) {
                                    Ok(resources) => resources,
                                    Err(error) => {
                                        return vec![message::MainMessage::SetErrorEngine(
                                            error.to_string(),
                                        )]
                                    }
                                };
                            self.resources = Some(resources);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }

    fn retrieve_stuffs(&mut self) -> Vec<message::MainMessage> {
        if self.stuffs.is_none() {
            if let Some(get_stuffs_request) = self.get_stuffs_request.as_mut() {
                if let Some(data) = get_stuffs_request.try_recv() {
                    info!("Stuffs received");
                    match data {
                        Ok(stuffs_json_str) => {
                            let stuffs: Vec<entity::stuff::Stuff> =
                                match serde_json::from_str(&stuffs_json_str) {
                                    Ok(stuffs) => stuffs,
                                    Err(error) => {
                                        return vec![message::MainMessage::SetErrorEngine(
                                            error.to_string(),
                                        )]
                                    }
                                };
                            self.stuffs = Some(stuffs);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }

    fn retrieve_builds(&mut self) -> Vec<message::MainMessage> {
        if self.builds.is_none() {
            if let Some(get_builds_request) = self.get_builds_request.as_mut() {
                if let Some(data) = get_builds_request.try_recv() {
                    info!("Builds received");
                    match data {
                        Ok(builds_json_str) => {
                            let builds: Vec<entity::build::Build> =
                                match serde_json::from_str(&builds_json_str) {
                                    Ok(builds) => builds,
                                    Err(error) => {
                                        return vec![message::MainMessage::SetErrorEngine(
                                            error.to_string(),
                                        )]
                                    }
                                };
                            self.builds = Some(builds);
                        }
                        Err(error) => {
                            return vec![message::MainMessage::SetErrorEngine(error.to_string())];
                        }
                    }
                }
            };
        }

        vec![]
    }
}

impl Engine for LoadZoneEngine {
    fn tick(&mut self) -> Vec<crate::message::MainMessage> {
        let mut messages = vec![];

        messages.extend(self.make_tiles_request());
        messages.extend(self.make_player_request());
        messages.extend(self.retrieve_player());
        messages.extend(self.make_player_avatar_request());
        messages.extend(self.retrieve_player_avatar());

        messages.extend(self.make_zone_request());
        messages.extend(self.make_characters_request());
        messages.extend(self.make_avatars_zone_thumbs_requests());
        messages.extend(self.make_resources_request());
        messages.extend(self.make_stuffs_request());
        messages.extend(self.make_builds_request());

        messages.extend(self.retrieve_tiles());
        messages.extend(self.retrieve_zone());
        messages.extend(self.retrieve_characters());
        messages.extend(self.retrieve_avatars_zone_thumbs());
        messages.extend(self.retrieve_resources());
        messages.extend(self.retrieve_stuffs());
        messages.extend(self.retrieve_builds());

        if let (
            Some(player),
            Some(player_avatar),
            Some(map),
            Some(characters),
            Some(resources),
            Some(stuffs),
            Some(builds),
        ) = (
            self.player.as_mut(),
            self.player_avatar.as_mut(),
            self.zone.as_mut(),
            self.characters.as_mut(),
            self.resources.as_mut(),
            self.stuffs.as_mut(),
            self.builds.as_mut(),
        ) {
            // Build final state
            let state = super::zone::state::ZoneState::new(
                &self.graphics,
                map.clone(),
                characters.clone(),
                player.clone(),
                stuffs.clone(),
                resources.clone(),
                builds.clone(),
            );

            messages.push(message::MainMessage::SetZoneEngine(
                self.client.clone(),
                state,
            ));
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                ui.colored_label(egui::Color32::LIGHT_GRAY, "Chargement ...");
            });
        });
        egui_macroquad::draw();

        messages
    }
}
