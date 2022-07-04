use crate::{engine::zone::state::ZoneState, util::texture_from_cache_or_from_file};

use super::Graphics;

pub async fn fill_avatars_from_zone_state(
    state: &ZoneState,
    mut graphics: Graphics,
) -> Result<Graphics, String> {
    // Add player avatar to zone graphics
    let player_avatar_uuid = state.player.private_avatar_uuid();
    let player_avatar_texture = texture_from_cache_or_from_file(&format!(
        "media/character_avatar__original__{}.png",
        player_avatar_uuid
    ))
    .await?;
    graphics.add_avatar_texture(player_avatar_uuid, player_avatar_texture);

    // Add characters avatars to zone graphics
    // FIXME NOW

    Ok(graphics)
}
