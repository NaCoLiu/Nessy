use crate::pages::{login, reg};
use eframe::egui;
use rust_i18n::t;
rust_i18n::i18n!("locales");
const SUPPORTED_LANGUAGES: &[&str] = &["en", "zh"];
#[derive(PartialEq)]
pub enum Tab {
    Login,
    Register,
}

pub struct TabState {
    pub username: String,
    pub password: String,
    pub current_tab: Tab,
    pub locale: String,
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            current_tab: Tab::Login,
            locale: "en".to_string()
        }
    }
}

pub fn render_tabs(ui: &mut egui::Ui, state: &mut TabState,) {
    ui.horizontal(|ui| {
        if ui
            .selectable_label(matches!(state.current_tab, Tab::Login), t!("login_tab"))
            .clicked()
        {
            state.current_tab = Tab::Login;
        }
        if ui
            .selectable_label(
                matches!(state.current_tab, Tab::Register),
                t!("register_tab"),
            )
            .clicked()
        {
            state.current_tab = Tab::Register;
        }

        ui.add_space(5.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let lang_key = format!("{}", state.locale);
            egui::ComboBox::from_id_salt("language_selector")
                .selected_text(t!(&lang_key))
                .show_ui(ui, |ui| {
                    for lang in SUPPORTED_LANGUAGES {
                        let lang_key = format!("{}", lang);
                        if ui
                            .selectable_label(state.locale == *lang, t!(&lang_key))
                            .clicked()
                        {
                           
                            state.locale = lang.to_string();
                            rust_i18n::set_locale(lang);
                        }
                    }
                });
            ui.label(egui_material_icons::icons::ICON_LANGUAGE);
        });
    });

    ui.separator();
    ui.add_space(10.0);

    // 渲染当前选中的标签页
    match state.current_tab {
        Tab::Login => login::render_login_tab(ui, state),
        Tab::Register => reg::render_register_tab(ui),
    }
}
