use macroquad::prelude::*;

use crate::{entity::description::RequestClicks, event};

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
pub fn require_resume_text_event() -> String {
    serde_json::to_string(&event::ZoneEvent {
        event_type_name: String::from(event::CLIENT_REQUIRE_NEW_RESUME_TEXT),
        event_type: event::ZoneEventType::ClientRequireResumeText,
    })
    .unwrap()
}

pub fn player_move_event(state: &super::state::ZoneState) -> String {
    serde_json::to_string(&event::ZoneEvent {
        event_type_name: String::from(event::PLAYER_MOVE),
        event_type: event::ZoneEventType::PlayerMove {
            to_row_i: state.player.zone_row_i,
            to_col_i: state.player.zone_col_i,
            character_id: state.player.id.clone(),
        },
    })
    .unwrap()
}

pub fn click_action_event(request_clicks: &RequestClicks, row_i: i16, col_i: i16) -> String {
    serde_json::to_string(&event::ZoneEvent {
        event_type_name: String::from(event::CLICK_ACTION_EVENT),
        event_type: event::ZoneEventType::ClickActionEvent {
            action_type: request_clicks.action_type.clone(),
            action_description_id: request_clicks.action_description_id.clone(),
            row_i: row_i,
            col_i: col_i,
        },
    })
    .unwrap()
}

pub fn in_area(row_i: i32, col_i: i32, draw_area: &((i32, i32), (i32, i32))) -> bool {
    let ((row_min, col_min), (row_max, col_max)) = draw_area;
    row_i >= *row_min && row_i <= *row_max && col_i >= *col_min && col_i <= *col_max
}
