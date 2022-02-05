use macroquad::prelude::*;
use quad_net::http_request::Request;
use serde_json::Value;

use crate::{client, entity, message, zone};

use super::Engine;

pub struct LoadZoneEngine {
    graphics: crate::graphics::Graphics,
    client: client::Client,
    character_id: String,
    get_player_request: Option<Request>,
    player: Option<entity::character::Character>,
    get_zone_request: Option<Request>,
    zone: Option<zone::map::ZoneMap>,
    get_characters_request: Option<Request>,
    characters: Option<Vec<entity::character::Character>>,
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
            get_zone_request: None,
            zone: None,
            get_characters_request: None,
            characters: None,
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

    fn retrieve_player(&mut self) -> Vec<message::MainMessage> {
        if let Some(get_player_request) = self.get_player_request.as_mut() {
            if let Some(data) = get_player_request.try_recv() {
                info!("Player character received");
                match data {
                    Ok(character_json_str) => {
                        let character: entity::character::Character =
                            match serde_json::from_str(&character_json_str) {
                                Ok(character) => character,
                                Err(error) => {
                                    return vec![message::MainMessage::SetErrorEngine(
                                        error.to_string(),
                                    )]
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

    fn retrieve_zone(&mut self) -> Vec<message::MainMessage> {
        if self.zone.is_none() {
            if let Some(get_zone_request) = self.get_zone_request.as_mut() {
                if let Some(data) = get_zone_request.try_recv() {
                    info!("Zone received");
                    match data {
                        Ok(zone_str) => {
                            let source_value: Value = serde_json::from_str(&zone_str).unwrap();
                            let source = source_value["raw_source"].as_str().unwrap();
                            let map: zone::map::ZoneMap = match zone::load::from_txt_map(
                                source,
                                self.graphics.tile_width,
                                self.graphics.tile_height,
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
                    self.get_zone_request = None;
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
    fn run(&mut self) -> Vec<crate::message::MainMessage> {
        let mut messages = vec![];

        messages.extend(self.make_player_request());
        messages.extend(self.retrieve_player());

        messages.extend(self.make_zone_request());
        messages.extend(self.make_characters_request());
        messages.extend(self.make_resources_request());
        messages.extend(self.make_stuffs_request());
        messages.extend(self.make_builds_request());

        messages.extend(self.retrieve_zone());
        messages.extend(self.retrieve_characters());
        messages.extend(self.retrieve_resources());
        messages.extend(self.retrieve_stuffs());
        messages.extend(self.retrieve_builds());

        if let (
            Some(player),
            Some(map),
            Some(characters),
            Some(resources),
            Some(stuffs),
            Some(builds),
        ) = (
            self.player.as_mut(),
            self.zone.as_mut(),
            self.characters.as_mut(),
            self.resources.as_mut(),
            self.stuffs.as_mut(),
            self.builds.as_mut(),
        ) {
            let state = super::zone::state::ZoneState::new(
                map.clone(),
                characters.clone(),
                player.clone(),
                stuffs.clone(),
                resources.clone(),
                builds.clone(),
            );
            messages.push(message::MainMessage::SetEngine(Box::new(
                super::zone::ZoneEngine::new(self.graphics.clone(), state),
            )));
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(&egui_ctx, |ui| {
                ui.colored_label(egui::Color32::LIGHT_GRAY, "Chargement ...");
            });
        });

        messages
    }
}