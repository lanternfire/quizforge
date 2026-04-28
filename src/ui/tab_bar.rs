use eframe::egui;
use egui::{Color32, Rounding};
use super::Tab;

pub fn render(ui: &mut egui::Ui, active_tab: &mut Tab, dark_mode: &mut bool) {
    let visuals = &ui.style().visuals;
    // 浅色模式下统一使用 #B3D9FF，深色模式跟随主题
    let btn_color = if *dark_mode {
        visuals.widgets.noninteractive.bg_fill
    } else {
        Color32::from_rgb(0xB3, 0xD9, 0xFF)
    };

    ui.horizontal(|ui| {
        let btn_width = 80.0;
        let btn_height = 24.0;

        let main_btn = egui::Button::new("主界面")
            .fill(btn_color)
            .rounding(Rounding::same(6.0))
            .min_size(egui::vec2(btn_width, btn_height));
        if ui.add(main_btn).clicked() {
            *active_tab = Tab::Main;
        }

        let example_btn = egui::Button::new("帮助中心")
            .fill(btn_color)
            .rounding(Rounding::same(6.0))
            .min_size(egui::vec2(btn_width + 20.0, btn_height));
        if ui.add(example_btn).clicked() {
            *active_tab = Tab::Example;
        }

        let spring = ui.available_width() - 80.0;
        if spring > 0.0 {
            ui.add_space(spring);
        }

        let toggle_text = if *dark_mode { "浅色模式" } else { "深色模式" };
        let toggle_btn = egui::Button::new(toggle_text).fill(btn_color);
        if ui.add(toggle_btn).clicked() {
            *dark_mode = !*dark_mode;
        }
    });

    ui.separator();
    ui.add_space(5.0);
}