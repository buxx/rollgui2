use crate::{entity, zone};
pub struct ZoneState {
    pub map: zone::map::ZoneMap,
    pub characters: Vec<entity::character::Character>,
    pub player: entity::character::Character,
}

impl ZoneState {
    pub fn new(
        map: zone::map::ZoneMap,
        characters: Vec<entity::character::Character>,
        player: entity::character::Character,
    ) -> Self {
        Self {
            map,
            characters,
            player,
        }
    }
}
