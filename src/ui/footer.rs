use eframe::egui;
use rust_i18n::t;
use std::env;

pub fn render_footer(ui: &mut egui::Ui) {
    ui.add_space(5.0);
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.add_space(2.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            ui.label("© 2025");
            ui.label(format!(
                "{} V{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.hyperlink_to(
                    format!("{} {}", egui_material_icons::icons::ICON_UPDATE, t!("Update")),
                    "https://github.com/yourusername/yourproject",
                );
            });
        });
    });
}
