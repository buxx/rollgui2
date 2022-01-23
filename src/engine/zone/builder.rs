use crate::{config, graphics, hardcoded, zone};

use super::{state::ZoneState, ZoneEngine};

pub fn build_zone_engine(
    graphics: graphics::Graphics,
    _config: config::Config,
) -> Result<ZoneEngine, String> {
    // config will be used to create the http/ws client and grab zone source, characters, etc.
    let map = zone::load::from_txt_map(hardcoded::get_map_source())?;
    let characters = vec![];
    let state = ZoneState::new(map, characters);
    Ok(ZoneEngine::new(graphics, state))
}
