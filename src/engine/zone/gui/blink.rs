use macroquad::prelude::*;

pub struct BlinkingIcon {
    frame_counter: i64,
    source: Rect,
    frame_target: i64,
    visible: bool,
    visible_duration: i64,
}

impl BlinkingIcon {
    pub fn new(source: Rect) -> Self {
        Self {
            frame_counter: 0,
            source,
            frame_target: 60 * 2,
            visible: true,
            visible_duration: 15,
        }
    }

    pub fn update(&mut self, _frame_i: i64) -> bool {
        self.frame_counter += 1;
        if self.frame_counter % self.visible_duration == 0 {
            self.visible = !self.visible;
        }

        self.frame_counter == self.frame_target
    }

    pub fn source(&self) -> Rect {
        self.source
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
