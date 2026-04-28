use eframe::egui;
use crate::localization::Locale;

pub fn render(ui: &mut egui::Ui, dark_mode: bool, locale: &Locale) {
    let heading_color = if dark_mode {
        egui::Color32::from_rgb(130, 190, 230)
    } else {
        egui::Color32::from_rgb(46, 125, 166)
    };
    let text_color = if dark_mode {
        egui::Color32::from_rgb(240, 240, 240)
    } else {
        egui::Color32::from_rgb(0, 0, 0)
    };
    let hint_color = if dark_mode {
        egui::Color32::from_rgb(180, 180, 180)
    } else {
        egui::Color32::from_rgb(100, 100, 100)
    };

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show(ui, |ui| {
            ui.heading(
                egui::RichText::new(locale.tr("tab_help"))
                    .color(heading_color)
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(10.0);

            // 使用流程
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new(locale.tr("help_usage_title")).size(16.0).color(text_color).strong());
                ui.add_space(5.0);
                ui.label(locale.tr("help_usage_step1"));
                ui.label(locale.tr("help_usage_step2"));
                ui.label(locale.tr("help_usage_step3"));
            });

            ui.add_space(10.0);

            // AI 提示词模板
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(locale.tr("help_ai_title")).size(16.0).color(text_color).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(
                            egui::Button::new(
                                egui::RichText::new(locale.tr("copy_prompt_btn")).color(egui::Color32::WHITE)
                            )
                            .fill(if dark_mode { egui::Color32::from_rgb(60, 130, 180) } else { egui::Color32::from_rgb(0xC0, 0xC0, 0xFF) })
                            .min_size(egui::vec2(100.0, 26.0))
                            .rounding(6.0)
                        ).clicked() {
                            let prompt = locale.tr("ai_prompt_template").to_string();
                            ui.ctx().output_mut(|o| o.copied_text = prompt.to_string());
                        }
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(locale.tr("help_copy_desc"))
                        .size(13.0).color(hint_color)
                );
                ui.add_space(4.0);

                let prompt_display = locale.tr("ai_prompt_display").to_string();
                ui.add(
                    egui::TextEdit::multiline(&mut prompt_display.to_string())
                        .desired_width(ui.available_width())
                        .desired_rows(12)
                        .font(egui::TextStyle::Monospace)
                        .interactive(false)
                        .text_color(text_color)
                );
            });
        });
}