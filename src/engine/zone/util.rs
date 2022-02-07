use crate::event;

pub fn require_around_event(state: &super::state::ZoneState) -> String {
    serde_json::to_string(&event::ZoneEvent {
        event_type_name: String::from(event::CLIENT_REQUIRE_AROUND),
        event_type: event::ZoneEventType::ClientRequireAround {
            zone_row_i: state.player.zone_row_i as i32,
            zone_col_i: state.player.zone_col_i as i32,
            character_id: state.player.id.clone(),
        },
    })
    .unwrap()
}
