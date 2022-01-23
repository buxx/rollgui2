use crate::{graphics, zone};

use super::{state::ZoneState, ZoneEngine};

pub fn build_zone_engine(graphics: graphics::Graphics, source: zone::ZoneMapSource) -> ZoneEngine {
    let map = zone::map::ZoneMap::new(source);
    let state = ZoneState::new(map);
    ZoneEngine::new(graphics, state)
}
