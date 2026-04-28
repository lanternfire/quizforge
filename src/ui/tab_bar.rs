use eframe::egui;
use egui::{Color32, Rounding};
use super::Tab;
use crate::localization::Locale;

pub fn render(ui: &mut egui::Ui, active_tab: &mut Tab, dark_mode: &mut bool, current_lang: &mut String, locale: &mut Locale) {
    let visuals = &ui.style().visuals;
    let btn_color = if *dark_mode {
        visuals.widgets.noninteractive.bg_fill
    } else {
        Color32::from_rgb(0xFF, 0xE5, 0xFF)
    };

    ui.horizontal(|ui| {
        // 左侧标签按钮
        let btn_width = 80.0;
        let btn_height = 24.0;

        let main_btn = egui::Button::new(locale.tr("tab_main"))
            .fill(btn_color)
            .rounding(Rounding::same(6.0))
            .min_size(egui::vec2(btn_width, btn_height));
        if ui.add(main_btn).clicked() {
            *active_tab = Tab::Main;
        }

        let help_btn = egui::Button::new(locale.tr("tab_help"))
            .fill(btn_color)
            .rounding(Rounding::same(6.0))
            .min_size(egui::vec2(btn_width + 20.0, btn_height));
        if ui.add(help_btn).clicked() {
            *active_tab = Tab::Example;
        }

        // 将剩余空间推到右侧，让后面的控件右对齐
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let right_btn_width = 65.0;
            let combo_width = 80.0;
            let spacing = 8.0;

            // 深色模式切换按钮
            let dark_text = if *dark_mode { locale.tr("light_mode_switch") } else { locale.tr("dark_mode_switch") };
            if ui.add(
                egui::Button::new(dark_text)
                    .fill(btn_color)
                    .min_size(egui::vec2(right_btn_width, btn_height))
            ).clicked() {
                *dark_mode = !*dark_mode;
            }

            ui.add_space(spacing);

            // 语言切换下拉
            let current_lang_name = if *current_lang == "zh_cn" { "中文" } else { "English" };
            egui::ComboBox::from_id_source("lang_selector")
                .width(combo_width)
                .selected_text(current_lang_name)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(*current_lang == "zh_cn", "中文").clicked() {
                        *current_lang = "zh_cn".to_string();
                        *locale = Locale::load("zh_cn");
                    }
                    if ui.selectable_label(*current_lang == "en_us", "English").clicked() {
                        *current_lang = "en_us".to_string();
                        *locale = Locale::load("en_us");
                    }
                });
        });
    });

    ui.separator();
    ui.add_space(5.0);
}