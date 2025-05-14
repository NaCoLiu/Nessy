use eframe::egui;
use std::{env, sync::Arc};

mod ui;
use ui::tabs::TabState;
mod pages;
mod config;

rust_i18n::i18n!("locales");

fn main() -> eframe::Result<()> {
    // 使用配置中的语言设置初始化
    let language = config::get_language();
    rust_i18n::set_locale(&language);

    let app_name = env!("CARGO_PKG_NAME");
    let mut options = eframe::NativeOptions::default();
    let icon_data = eframe::icon_data::from_png_bytes(include_bytes!("../assets/logo.png"))
        .expect("The icon data must be valid");
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([300.0, 260.0])
        .with_maximize_button(false)
        .with_resizable(false);
    options.viewport.icon = Some(Arc::new(icon_data));
    eframe::run_native(
        app_name,
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            egui_material_icons::initialize(&cc.egui_ctx);
            Ok(Box::new(Welcome::default()))
        }),
    )
}

struct Welcome {
    tab_state: TabState,
}

impl Default for Welcome {
    fn default() -> Self {
        Self {
            tab_state: TabState::default(),
        }
    }
}

impl eframe::App for Welcome {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(5.0);
            ui::tabs::render_tabs(ui, &mut self.tab_state);
            ui::footer::render_footer(ui);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "chinese_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/MapleMono-NF-CN-Regular.ttf"
        ))
        .into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "chinese_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("chinese_font".to_owned());
    let mut style = (*ctx.style()).clone();
    style.interaction.selectable_labels = false;
    ctx.set_style(style);
    ctx.set_fonts(fonts);
}
