use eframe::egui;
use rust_i18n::t;

pub fn render_register_tab(ui: &mut egui::Ui) {
    ui.label(t!("register_info"));
}
