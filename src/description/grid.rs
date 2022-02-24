impl super::UiDescription {
    pub fn draw_grid(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut super::UiDescriptionState,
    ) -> Option<super::UiDescriptionEvent> {
        let mut event = None;

        ui.horizontal_wrapped(|ui| {
            self.draw_big_button = true;
            for (i, part) in self.description.items.iter().enumerate() {
                match self.draw_part(ui, part, state) {
                    Some(event_) => event = Some(event_),
                    None => {}
                }
            }
            self.draw_big_button = false;
        });

        if self.description.footer_links.len() > 0 {
            for footer_link in &self.description.footer_links {
                if let Some(event_) = self.draw_button(ui, footer_link, state) {
                    event = Some(event_)
                }
            }
        }

        event
    }
}
