use macroquad::prelude::*;

use crate::{
    engine::root::state,
    graphics::Graphics,
    ui::utils::{egui_scale, is_mobile},
    util::vname,
};
use egui::{self, TextureFilter};

pub fn ui(state: &mut state::RootState, graphics: &Graphics) -> Option<super::RootEvent> {
    let mut event = None;

    egui_macroquad::ui(|egui_ctx| {
        if state.first_frame {
            let mut style = (*egui_ctx.style()).clone();
            // TODO : with new egui, do https://discord.com/channels/900275882684477440/900275883124858921/938081008568377354
            style.override_text_style = Some(egui::TextStyle::Heading);
            egui_ctx.set_style(style);

            if let Some(image_data) = graphics.illustrations.get(&vname("root.png")) {
                let texture: egui::TextureHandle = egui_ctx.load_texture(
                    vname("root.png"),
                    image_data.clone(),
                    TextureFilter::Linear,
                );
                state.root_illustration = Some(texture);
            }
        }

        if let Some(texture_handler) = &state.root_illustration {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let image_width = screen_width() / egui_scale();
                let image_height = 700.0 / (1000.0 / image_width);
                ui.image(texture_handler, egui::Vec2::new(image_width, image_height))
            });
        }

        egui::Window::new("Se connecter / Créer un compte")
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -50.))
            .show(egui_ctx, |ui| {
                if state.loading {
                    ui.colored_label(egui::Color32::LIGHT_GRAY, "Chargement ...");
                } else {
                    if let Some(error_message) = &state.error_message {
                        ui.colored_label(egui::Color32::RED, error_message);
                    } else if let Some((home_message, color)) = &state.home_message {
                        ui.colored_label(*color, home_message);
                    } else {
                        ui.label("");
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("Login: ");
                    let login_input = ui.text_edit_singleline(&mut state.login);

                    if state.first_frame && !is_mobile() {
                        login_input.request_focus();
                    }

                    if login_input.gained_focus() {
                        event = Some(super::RootEvent::TextEditFocused(
                            super::RootTextInput::Login,
                        ));
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Mot de passe: ");
                    if ui
                        .add(egui::TextEdit::singleline(&mut state.password).password(true))
                        .gained_focus()
                    {
                        event = Some(super::RootEvent::TextEditFocused(
                            super::RootTextInput::Password,
                        ));
                    }
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.remember_me, "Rester connecté");
                });

                ui.horizontal(|ui| {
                    if ui.button("Se connecter").clicked() {
                        event = Some(super::RootEvent::DoLoginWithCredentials);
                    }

                    if ui.button("Mot de passe perdu").clicked() {
                        event = Some(super::RootEvent::GoToPasswordLost);
                    }
                    if ui.button("Créer un compte").clicked() {
                        event = Some(super::RootEvent::GoToCreateAccount);
                    }
                })
            });
    });

    event
}
