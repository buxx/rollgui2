impl super::ZoneEngine {
    pub fn camera_animations(&mut self) {
        let mut to_removes: Vec<usize> = vec![];

        for (i, animation) in self.camera_animations.iter_mut().enumerate() {
            if animation.update(self.frame_i) {
                to_removes.push(i);
            }
            animation.draw_in_camera(&self.graphics);
        }

        // Remove finished animations
        to_removes.sort();
        to_removes.reverse();
        for animation_i_to_remove in to_removes {
            self.camera_animations.remove(animation_i_to_remove);
        }
    }

    pub fn ui_animations(&mut self) {
        let mut to_removes: Vec<usize> = vec![];

        for (i, animation) in self.ui_animations.iter_mut().enumerate() {
            if animation.update(self.frame_i) {
                to_removes.push(i);
            }
            animation.draw_in_camera(&self.graphics);
        }

        // Remove finished animations
        to_removes.sort();
        to_removes.reverse();
        for animation_i_to_remove in to_removes {
            self.ui_animations.remove(animation_i_to_remove);
        }
    }
}
