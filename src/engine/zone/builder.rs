use crate::{config, entity, graphics, hardcoded, zone};

use super::{state::ZoneState, ZoneEngine};

pub fn build_zone_engine(
    graphics: graphics::Graphics,
    _config: config::Config,
) -> Result<ZoneEngine, String> {
    // config will be used to create the http/ws client and grab zone source, characters, etc.
    let map = zone::load::from_txt_map(
        hardcoded::get_map_source(),
        graphics.tile_width,
        graphics.tile_height,
    )?;
    let characters = vec![];
    // FIXME faked
    let player_zone_row_i = 0;
    let player_zone_col_i = 0;
    let player = entity::character::Character {
        id: "abc".to_string(),
        zone_row_i: player_zone_row_i,
        zone_col_i: player_zone_col_i,
        avatar_uuid: None,
        avatar_is_validated: false,
        display_x: player_zone_col_i as f32 * 32.,
        display_y: player_zone_row_i as f32 * 32.,
    };
    let state = ZoneState::new(map, characters, player);
    Ok(ZoneEngine::new(graphics, state))
}
