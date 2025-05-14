use eframe::egui;
use rust_i18n::t;
use crate::{config::set_credentials, ui::tabs::TabState};

pub fn render_login_tab(ui: &mut egui::Ui, state: &mut TabState) {
    ui.label(format!("{} {}", egui_material_icons::icons::ICON_EMAIL, t!("email")));
    ui.add_space(5.0);

    ui.add(
        egui::TextEdit
            ::singleline(&mut state.email)
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Body)
            .min_size([0.0, 24.0].into())
            .margin(egui::vec2(8.0, 4.0))
            .vertical_align(egui::Align::Center)
    );

    ui.add_space(10.0);
    ui.label(format!("{} {}", egui_material_icons::icons::ICON_LOCK, t!("password")));
    ui.add_space(5.0);
    ui.add(
        egui::TextEdit
            ::singleline(&mut state.password)
            .password(true)
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Body)
            .min_size([0.0, 24.0].into())
            .margin(egui::vec2(8.0, 4.0))
            .vertical_align(egui::Align::Center)
    );
    ui.add_space(20.0);
    ui.horizontal(|ui| {
        if ui.button(t!("login")).clicked() {
            let _ = set_credentials(&state.email, &state.password);
        }
       
    });
}
