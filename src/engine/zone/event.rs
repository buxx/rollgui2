use macroquad::prelude::*;

impl super::ZoneEngine {
    pub fn event(&mut self, event: crate::event::ZoneEvent) {
        match event.event_type {
            crate::event::ZoneEventType::ThereIsAround {
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
                self.current_action = Some(crate::action::Action::from_quick_action(quick_action));
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
