use macroquad::prelude::*;

impl super::UiDescription {
    pub fn draw_grid(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        egui::Grid::new("GRID").show(ui, |ui| {
            // TODO : use real window max size
            let columns = screen_width() as usize / super::BIG_BUTTON_SIZE.0 as usize;

            for (i, part) in self.description.items.iter().enumerate() {
                match self.draw_part(ui, part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
                if i % columns == 0 && i != 0 {
                    ui.end_row();
                }
            }
        });

        event
    }
}
