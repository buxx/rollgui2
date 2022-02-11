use macroquad::prelude::*;

use crate::{action, animation, event};

impl super::ZoneEngine {
    pub fn event(&mut self, event: crate::event::ZoneEvent) {
        match event.event_type {
            event::ZoneEventType::ThereIsAround {
                stuff_count,
                resource_count,
                build_count,
                character_count,
                quick_actions,
            } => {
                debug!("New quick actions ({})", quick_actions.len());
                self.quick_actions = quick_actions;
                self.update_current_action_according_new_quick_actions();
            }
            event::ZoneEventType::NewBuild {
                build,
                produced_resource_id,
                produced_stuff_id,
                producer_character_id,
            } => {
                let mut tile_id: Option<String> = None;
                if let Some(produced_stuff_id) = produced_stuff_id {
                    tile_id = Some(produced_stuff_id);
                } else if let Some(produced_resource_id) = produced_resource_id {
                    tile_id = Some(produced_resource_id);
                }

                if let Some(tile_id_) = tile_id {
                    let pop_animation = match animation::pop::TilePopAnimation::new(
                        &self.graphics,
                        &self.state.map,
                        &tile_id_,
                        build.row_i,
                        build.col_i,
                        // TODO: experimental
                        self.frame_i + 60,
                    ) {
                        Ok(pop_animation_) => pop_animation_,
                        Err(error) => {
                            error!("Error during pop animation : {}", error);
                            return;
                        }
                    };
                    self.animations.push(Box::new(pop_animation));

                    // Consider we can clean active exploitable tile (this can be wrong because an other action can been selected since)
                    if let Some(current_action) = &self.current_action {
                        for (i, exploitable_tile) in
                            current_action.exploitable_tiles.iter().enumerate()
                        {
                            if exploitable_tile.zone_row_i == build.row_i
                                && exploitable_tile.zone_col_i == build.col_i
                                && self.pending_exploitable_tiles.contains(&i)
                            {
                                self.pending_exploitable_tiles.retain(|p| p != &i);
                            }
                        }
                    }
                }

                self.state.builds.insert(build.id, build);
            }
            _ => {}
        }
    }

    pub fn update_current_action_according_new_quick_actions(&mut self) {
        if let Some(action) = &self.current_action {
            if let Some(quick_action) = self
                .quick_actions
                .iter()
                .find(|quick_action| quick_action.base_url == action.post_url)
            {
                self.current_action = Some(action::Action::from_quick_action(quick_action));
                self.selected_quick_action = Some(
                    self.quick_actions
                        .iter()
                        .position(|q| q.base_url == quick_action.base_url)
                        .unwrap(),
                );
            } else {
                self.current_action = None;
                self.selected_quick_action = None;
            }
        }
    }
}
