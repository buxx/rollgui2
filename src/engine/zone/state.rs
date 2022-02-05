use std::collections::HashMap;

use crate::{entity, zone};
use macroquad::prelude::*;

pub struct ZoneState {
    pub map: zone::map::ZoneMap,
    pub characters: Vec<entity::character::Character>,
    pub player: entity::character::Character,
    pub player_display: PlayerDisplay,
    pub stuffs: HashMap<i32, entity::stuff::Stuff>,
    pub resources: Vec<entity::resource::Resource>,
    pub builds: HashMap<i32, entity::build::Build>,
}

impl ZoneState {
    pub fn new(
        map: zone::map::ZoneMap,
        characters: Vec<entity::character::Character>,
        player: entity::character::Character,
        stuffs: Vec<entity::stuff::Stuff>,
        resources: Vec<entity::resource::Resource>,
        builds: Vec<entity::build::Build>,
    ) -> Self {
        let mut builds_ = HashMap::new();
        for build in builds {
            builds_.insert(build.id.clone(), build);
        }

        let mut stuffs_ = HashMap::new();
        for stuff in stuffs {
            stuffs_.insert(stuff.id.clone(), stuff);
        }

        // FIXME
        let player_display = PlayerDisplay::default();

        Self {
            map,
            characters,
            player,
            player_display,
            stuffs: stuffs_,
            resources,
            builds: builds_,
        }
    }
}

pub struct PlayerDisplay {
    pub position: Vec2,
    pub velocity: Vec2,
    pub running: Option<super::PlayerRunning>,
}

impl Default for PlayerDisplay {
    fn default() -> Self {
        Self {
            position: Vec2::new(0., 0.),
            velocity: Vec2::new(0., 0.),
            running: None,
        }
    }
}
