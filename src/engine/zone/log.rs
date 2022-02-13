use macroquad::prelude::*;

const LOG_BOX_MARGIN: f32 = 10.0;
const LOG_LINE_HEIGHT: f32 = 20.0;
const LOG_LINE_FONT_SIZE: f32 = 20.0;
const LOG_BOX_HEIGHT: f32 = super::DISPLAY_USER_LOG_COUNT as f32 * LOG_LINE_HEIGHT;
const LOG_BOX_WIDTH: f32 = 230.0;

pub struct UserLog {
    level: UserLogLevel,
    message: String,
}

pub enum UserLogLevel {
    Info,
    Error,
}

impl UserLog {
    pub fn info(message: String) -> Self {
        Self {
            level: UserLogLevel::Info,
            message,
        }
    }
    pub fn error(message: String) -> Self {
        Self {
            level: UserLogLevel::Error,
            message,
        }
    }

    pub fn new(message: String, message_level: UserLogLevel) -> UserLog {
        Self {
            level: message_level,
            message,
        }
    }
}

impl std::fmt::Display for UserLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserLogLevel::Info => f.write_str("INFO"),
            UserLogLevel::Error => f.write_str("ERROR"),
        }
    }
}

impl super::ZoneEngine {
    pub fn draw_user_logs(&self) {
        let draw_log_box_x = 0. + LOG_BOX_MARGIN;
        let draw_log_box_y = screen_height() - LOG_BOX_MARGIN - LOG_BOX_HEIGHT;

        draw_rectangle(
            draw_log_box_x,
            draw_log_box_y,
            LOG_BOX_WIDTH,
            LOG_BOX_HEIGHT,
            GRAY,
        );

        let start_draw_message_x = draw_log_box_x + 10.;
        let start_draw_message_y = draw_log_box_y + 15.;
        for (i, user_log) in self.user_logs.iter().rev().enumerate() {
            let draw_x = start_draw_message_x;
            let draw_y = start_draw_message_y + (i as f32 * LOG_LINE_HEIGHT);
            let color = match user_log.level {
                UserLogLevel::Info => BLACK,
                UserLogLevel::Error => RED,
            };
            draw_text(&user_log.message, draw_x, draw_y, LOG_LINE_FONT_SIZE, color);
        }
    }
}
