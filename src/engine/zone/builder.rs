use crate::{config, graphics, zone};

use super::{state::ZoneState, ZoneEngine};

pub fn build_zone_engine(graphics: graphics::Graphics, _config: config::Config) -> ZoneEngine {
    // config will be used to create the http/ws client and grab zone source, characters, etc.
    let map = zone::map::ZoneMap::new(vec![]);
    let characters = vec![];
    let state = ZoneState::new(map, characters);
    ZoneEngine::new(graphics, state)
}
