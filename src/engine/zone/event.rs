use macroquad::prelude::*;

use crate::{
    action,
    animation::{self, visible::VisibleAnimation},
    engine::zone::{gui::chat::model::Message, resume::CharacterResume},
    entity, event,
};

impl super::ZoneEngine {
    pub fn event(&mut self, event: crate::event::ZoneEvent) {
        // This a hack because click used for build but not good reliable things
        self.pending_request_clicks = None;
        debug!("Event received : {}", event.event_type_name);

        match event.event_type {
            event::ZoneEventType::ThereIsAround {
                stuff_count: _,
                resource_count: _,
                build_count: _,
                character_count: _,
                quick_actions,
            } => {
                self.quick_actions = quick_actions;
                self.quick_action_x_offset = None;
                // Keep only rollgui2 quick actions (where there is exploitable tiles)
                self.quick_actions
                    .retain(|a| a.exploitable_tiles.len() != 0 || a.direct_action);
                self.update_current_action_according_new_quick_actions();
            }
            event::ZoneEventType::RemoveBuild {
                zone_row_i,
                zone_col_i,
            } => {
                self.state
                    .builds
                    .retain(|k, _| k != &(zone_row_i, zone_col_i));
            }
            event::ZoneEventType::NewBuild {
                build,
                produced_resource_id,
                produced_stuff_id,
                producer_character_id: _,
            } => {
                let mut tile_id: Option<String> = None;
                if let Some(produced_stuff_id) = produced_stuff_id {
                    tile_id = Some(produced_stuff_id);
                } else if let Some(produced_resource_id) = produced_resource_id {
                    tile_id = Some(produced_resource_id);
                }

                if let Some(tile_id_) = tile_id {
                    let pop_animation = match animation::pop::TilePopAnimation::new(
                        &self.graphics,
                        &self.state.map,
                        &tile_id_,
                        build.row_i,
                        build.col_i,
                    ) {
                        Ok(pop_animation_) => pop_animation_,
                        Err(error) => {
                            error!("Error during pop animation : {}", error);
                            return;
                        }
                    };
                    self.camera_animations.push(Box::new(pop_animation));

                    // Consider we can clean active exploitable tile (this can be wrong because an other action can been selected since)
                    if let Some(current_action) = &self.current_action {
                        for (i, exploitable_tile) in
                            current_action.exploitable_tiles.iter().enumerate()
                        {
                            if exploitable_tile.zone_row_i == build.row_i
                                && exploitable_tile.zone_col_i == build.col_i
                                && self.pending_exploitable_tiles.contains(&i)
                            {
                                self.pending_exploitable_tiles.retain(|p| p != &i);
                            }
                        }
                    }
                }

                self.state.builds.insert((build.row_i, build.col_i), build);
                self.user_logs.push(super::log::UserLog::new(
                    "Un bâtiment a été construit".to_string(),
                    super::log::UserLogLevel::Info,
                ));
            }
            event::ZoneEventType::CharacterEnter {
                zone_row_i,
                zone_col_i,
                character_id,
            } => {
                self.state.characters.insert(
                    character_id.clone(),
                    entity::character::Character::minimal(
                        character_id.clone(),
                        zone_row_i,
                        zone_col_i,
                    ),
                );
                self.user_logs.push(super::log::UserLog::new(
                    "Un personnage vient d'arriver".to_string(),
                    super::log::UserLogLevel::Info,
                ));
            }
            event::ZoneEventType::CharacterExit { character_id } => {
                self.state.characters.remove(&character_id);
            }
            event::ZoneEventType::PlayerMove {
                to_row_i,
                to_col_i,
                character_id,
            } => {
                if let Some(character) = self.state.characters.get_mut(&character_id) {
                    character.zone_row_i = to_row_i;
                    character.zone_col_i = to_col_i;
                }
            }
            event::ZoneEventType::ZoneTileReplace {
                row_i,
                col_i,
                new_tile_id,
            } => {
                self.state.map.replace_tile(row_i, col_i, new_tile_id);
            }
            event::ZoneEventType::ZoneGroundResourceRemoved {
                row_i,
                col_i,
                resource_id,
            } => {
                if let Some(resources) = self.state.resources.get_mut(&(row_i, col_i)) {
                    resources.retain(|r| !(r.id == resource_id))
                }
            }
            event::ZoneEventType::ZoneGroundStuffRemoved { stuff_id } => {
                self.state.stuffs.remove(&stuff_id);
            }
            event::ZoneEventType::ZoneGroundResourceAdded {
                row_i,
                col_i,
                resource_id,
            } => self
                .state
                .resources
                .entry((row_i, col_i))
                .or_insert(vec![])
                .push(entity::resource::Resource {
                    id: resource_id,
                    zone_row_i: row_i,
                    zone_col_i: col_i,
                }),
            event::ZoneEventType::ZoneGroundStuffAdded {
                id_,
                stuff_id,
                zone_row_i,
                zone_col_i,
                classes,
            } => {
                self.state.stuffs.insert(
                    id_,
                    entity::stuff::Stuff {
                        id: id_,
                        stuff_id,
                        zone_row_i,
                        zone_col_i,
                        classes,
                    },
                );
            }
            event::ZoneEventType::NewResumeText { resume } => {
                match CharacterResume::from_resume_texts(resume) {
                    Ok(resume_) => {
                        // For each item which change, do pop animation
                        if let Some(before) = &self.resume {
                            self.blinking_icons
                                .extend(before.icons_from_compare(&resume_));
                        }

                        self.resume = Some(resume_);
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }
            }
            event::ZoneEventType::TopBarMessage { message, type_ } => {
                let message_level = match type_ {
                    event::TopBarMessageType::NORMAL => super::log::UserLogLevel::Info,
                    event::TopBarMessageType::ERROR => super::log::UserLogLevel::Error,
                };
                self.user_logs
                    .push(super::log::UserLog::new(message, message_level));
            }
            event::ZoneEventType::NewChatMessage {
                character_id,
                message,
                system,
                silent,
            } => {
                if let Some(character_id) = character_id {
                    if let Some(character) = self.state.characters.get(&character_id) {
                        self.chat_state
                            .add_character_message(Message::character(message));

                        let source = Rect {
                            x: 352.,
                            y: 192.,
                            w: 32.,
                            h: 32.,
                        };
                        let dest = self.zone_position_to_screen_position(
                            (character.zone_row_i - 1) as f32 - 0.5,
                            character.zone_col_i as f32 + 0.5,
                        );
                        self.ui_animations.push(Box::new(VisibleAnimation::new(
                            source,
                            dest,
                            self.frame_i + 60 * 5,
                        )));
                    }
                } else if system {
                    self.chat_state
                        .add_system_message(Message::system(message), silent);
                }
            }
            _ => {}
        }
    }

    pub fn update_current_action_according_new_quick_actions(&mut self) {
        if let Some(action) = &self.current_action {
            if let Some(quick_action) = self
                .quick_actions
                .iter()
                .find(|quick_action| quick_action.base_url == action.post_url)
            {
                self.current_action = Some(action::Action::from_quick_action(quick_action));
                self.selected_quick_action = Some(
                    self.quick_actions
                        .iter()
                        .position(|q| q.base_url == quick_action.base_url)
                        .unwrap(),
                );
            } else {
                self.current_action = None;
                self.selected_quick_action = None;
            }
        }
    }
}
