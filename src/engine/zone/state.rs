use crate::{entity, zone};
use macroquad::prelude::*;

pub struct ZoneState {
    pub map: zone::map::ZoneMap,
    pub characters: Vec<entity::character::Character>,
    pub player: entity::character::Character,
    pub player_display: PlayerDisplay,
}

impl ZoneState {
    pub fn new(
        map: zone::map::ZoneMap,
        characters: Vec<entity::character::Character>,
        player: entity::character::Character,
        player_display: PlayerDisplay,
    ) -> Self {
        Self {
            map,
            characters,
            player,
            player_display,
        }
    }
}

pub struct PlayerDisplay {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub moving: bool,
}

impl Default for PlayerDisplay {
    fn default() -> Self {
        Self {
            position: Vec2::new(0., 0.),
            rotation: 0.,
            velocity: Vec2::new(0., 0.),
            moving: false,
        }
    }
}
