use crate::engine::root::state;
use egui;

pub fn ui(state: &mut state::RootState) -> Option<super::RootEvent> {
    let mut event = None;

    egui_macroquad::ui(|egui_ctx| {
        if state.first_frame {
            let mut style = (*egui_ctx.style()).clone();
            // TODO : with new egui, do https://discord.com/channels/900275882684477440/900275883124858921/938081008568377354
            style.override_text_style = Some(egui::TextStyle::Heading);
            egui_ctx.set_style(style);
        }

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            if state.loading {
                ui.colored_label(egui::Color32::LIGHT_GRAY, "Chargement ...");
            } else {
                if let Some(error_message) = &state.error_message {
                    ui.colored_label(egui::Color32::RED, error_message);
                } else {
                    ui.label("");
                }
            }

            ui.horizontal(|ui| {
                ui.label("Login: ");
                ui.text_edit_singleline(&mut state.login);
            });

            ui.horizontal(|ui| {
                ui.label("Mot de passe: ");
                ui.add(egui::TextEdit::singleline(&mut state.password).password(true));
            });

            ui.horizontal(|ui| {
                if ui.button("Se connecter").clicked() {
                    event = Some(super::RootEvent::DoLogin);
                }
                if ui.button("Mot de passe perdu").clicked() {
                    // take some action here
                }
                if ui.button("Créer un compte").clicked() {
                    event = Some(super::RootEvent::GoToCreateAccount);
                }
            })
        });
    });

    event
}
