use ahash::AHashMap;

use crate::{entity, zone};
use macroquad::prelude::*;

pub struct ZoneState {
    pub map: zone::map::ZoneMap,
    pub characters: AHashMap<String, entity::character::Character>,
    pub player: entity::character::Character,
    pub player_display: CharacterDisplay,
    pub stuffs: AHashMap<i32, entity::stuff::Stuff>,
    pub resources: AHashMap<(i32, i32), Vec<entity::resource::Resource>>,
    pub builds: AHashMap<(i32, i32), entity::build::Build>,
}

impl ZoneState {
    pub fn new(
        graphics: &crate::graphics::Graphics,
        map: zone::map::ZoneMap,
        characters: Vec<entity::character::Character>,
        player: entity::character::Character,
        stuffs: Vec<entity::stuff::Stuff>,
        resources: Vec<entity::resource::Resource>,
        builds: Vec<entity::build::Build>,
    ) -> Self {
        let mut builds_ = AHashMap::new();
        for build in builds {
            builds_.insert((build.row_i, build.col_i), build);
        }

        let mut stuffs_ = AHashMap::new();
        for stuff in stuffs {
            stuffs_.insert(stuff.id.clone(), stuff);
        }

        let player_display = CharacterDisplay {
            position: Vec2::new(
                player.zone_col_i as f32 * graphics.tile_width,
                player.zone_row_i as f32 * graphics.tile_height,
            ),
            ..Default::default()
        };

        let mut resources_: AHashMap<(i32, i32), Vec<entity::resource::Resource>> = AHashMap::new();
        for resource in &resources {
            resources_
                .entry((resource.zone_row_i, resource.zone_col_i))
                .or_insert(vec![])
                .push(resource.clone());
        }
        let characters: AHashMap<String, entity::character::Character> = characters
            .iter()
            .map(|c| (c.id.clone(), c.clone()))
            .collect();

        Self {
            map,
            characters,
            player,
            player_display,
            stuffs: stuffs_,
            resources: resources_,
            builds: builds_,
        }
    }
}

pub struct CharacterDisplay {
    pub position: Vec2,
    pub velocity: Vec2,
    pub running: Option<super::PlayerRunning>,
}

impl Default for CharacterDisplay {
    fn default() -> Self {
        Self {
            position: Vec2::new(0., 0.),
            velocity: Vec2::new(0., 0.),
            running: None,
        }
    }
}
