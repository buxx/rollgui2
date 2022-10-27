use crate::{graphics::Graphics, ui::utils::egui_scale};
use egui::{Context, Pos2, Ui};
use macroquad::prelude::*;

use super::state::{DisplayState, State};

pub enum Display {
    Right,
    Bottom,
}

impl Display {
    pub fn from_env() -> Self {
        if screen_width() < screen_height() {
            Self::Bottom
        } else {
            Self::Right
        }
    }

    fn reference_side_size(&self) -> f32 {
        match self {
            Display::Right => screen_width(),
            Display::Bottom => screen_height(),
        }
    }

    fn covered_proportion(&self) -> f32 {
        if self.reference_side_size() < 1280. {
            0.5 / egui_scale()
        } else {
            0.3 / egui_scale()
        }
    }

    pub fn position(&self) -> Pos2 {
        let covered_proportion = self.covered_proportion();
        match self {
            Display::Right => Pos2::new(screen_width() * (1. - covered_proportion), 0.0),
            Display::Bottom => Pos2::new(0.0, screen_height() * (1. - covered_proportion)),
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Display::Right => (screen_width() - self.position().x),
            Display::Bottom => screen_width(),
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Display::Right => screen_height(),
            Display::Bottom => (screen_height() - self.position().y),
        }
    }
}

pub struct ChatDisplayer<'s> {
    state: &'s State,
}

impl<'s> ChatDisplayer<'s> {
    pub fn new(state: &'s State) -> Self {
        Self { state }
    }

    pub fn ui(&mut self, egui_ctx: &Context) -> DisplayState {
        let display = Display::from_env();
        let mut display_state = DisplayState::from_state(self.state);

        if display_state.input_focused
            && is_key_pressed(KeyCode::Enter)
            && display_state.input_value.len() != 0
        {
            display_state.input_validated = true;
        }

        // egui::Window::new("Zone")
        //     .resizable(false)
        //     .fixed_pos(display.position())
        //     // .fixed_size(display.size())
        //     .collapsible(false)
        //     .resizable(false)
        //     .show(egui_ctx, |ui| {
        //         egui::containers::ScrollArea::vertical().show(ui, |ui| {
        //             for message in self.state.messages() {
        //                 ui.label(format!("{}: {}", message.author_name(), message.message()));
        //             }
        //         });
        //         let added_input =
        //             ui.add(egui::TextEdit::singleline(&mut display_state.input_value));
        //         display_state.input_focused = added_input.has_focus();

        //         if self.state.request_focus() {
        //             added_input.request_focus()
        //         }
        //     });

        match display {
            Display::Right => {
                egui::SidePanel::right("chat_panel")
                    .default_width(display.width())
                    .min_width(display.width())
                    .max_width(display.width())
                    .resizable(false)
                    .show(egui_ctx, |ui| self.ui_content(&mut display_state, ui));
            }
            Display::Bottom => {
                egui::TopBottomPanel::bottom("chat_panel")
                    .default_height(display.height())
                    .resizable(false)
                    .show(egui_ctx, |ui| self.ui_content(&mut display_state, ui));
            }
        }

        display_state
    }

    fn ui_content(&self, display_state: &mut DisplayState, ui: &mut Ui) {
        let display = Display::from_env();

        ui.horizontal(|ui| {
            ui.add(egui::Button::new("Button1"));
            ui.add(egui::Button::new("Button2"));
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for message in self.state.messages() {
                ui.label(format!("{}: {}", message.author_name(), message.message()));
            }
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            let added_input = ui.add(
                egui::TextEdit::singleline(&mut display_state.input_value)
                    .desired_width(display.width() * 0.9),
            );
            display_state.input_focused = added_input.has_focus();

            if self.state.request_focus() {
                added_input.request_focus()
            }
        });
    }
}
