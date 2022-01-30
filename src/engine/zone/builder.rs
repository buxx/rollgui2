use crate::{config, entity, graphics, hardcoded, zone};
use macroquad::prelude::*;

pub fn build_zone_engine(
    graphics: graphics::Graphics,
    _config: config::Config,
) -> Result<super::ZoneEngine, String> {
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
    };
    let player_display = super::state::PlayerDisplay {
        position: Vec2::new(
            player_zone_col_i as f32 * 32.,
            player_zone_row_i as f32 * 32.,
        ),
        ..Default::default()
    };
    let builds = hardcoded::builds();
    let state = super::state::ZoneState::new(
        map,
        characters,
        player,
        player_display,
        vec![],
        vec![],
        builds,
    );
    Ok(super::ZoneEngine::new(graphics, state))
}
