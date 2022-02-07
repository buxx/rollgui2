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
                info!("New quick actions : {:?}", &quick_actions);
                self.quick_actions = quick_actions;
            }
            _ => {}
        }
    }
}
