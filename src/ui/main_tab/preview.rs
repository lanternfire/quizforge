use eframe::egui;
use egui::{Color32, Stroke};
use rfd::FileDialog;
use crate::templates::Template;

fn parse_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);
        Color32::from_rgb(r, g, b)
    } else {
        Color32::GRAY
    }
}

fn draw_shadow(painter: &egui::Painter, rect: egui::Rect, offset: egui::Vec2, color: Color32) {
    let shadow_rect = rect.translate(offset);
    painter.rect_filled(shadow_rect, 10.0, color);
    let inner = shadow_rect.translate(offset * 0.5);
    painter.rect_filled(inner, 10.0, Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), color.a() / 2));
}

pub fn render(ui: &mut egui::Ui, template: &dyn Template, dark_mode: bool, pending_import: &mut Option<String>) {
    let caption_color = if dark_mode {
        Color32::from_rgb(180, 180, 180)
    } else {
        Color32::from_rgb(100, 120, 140)
    };
    ui.add_space(10.0);
    ui.label(egui::RichText::new("预览图").size(14.0).color(caption_color));
    ui.add_space(5.0);

    if let Some(theme) = template.theme_colors() {
        let card_size = egui::vec2(300.0, 190.0);
        let (rect, _resp) = ui.allocate_at_least(card_size, egui::Sense::hover());
        let painter = ui.painter();

        draw_shadow(painter, rect, egui::vec2(3.0, 4.0), Color32::from_black_alpha(50));

        painter.rect_filled(rect, 12.0, parse_color(&theme.card_bg));
        painter.rect_stroke(rect, 12.0, Stroke::new(1.0, parse_color(&theme.border)));

        let title_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(18.0, 12.0),
            egui::vec2(260.0, 22.0),
        );
        painter.text(
            title_rect.left_center(),
            egui::Align2::LEFT_CENTER,
            "题匠",
            egui::FontId::proportional(15.0),
            parse_color(&theme.primary_deep),
        );

        let badge_rect = egui::Rect::from_min_size(
            rect.right_top() + egui::vec2(-75.0, 14.0),
            egui::vec2(55.0, 18.0),
        );
        painter.rect_filled(badge_rect, 9.0, parse_color(&theme.badge_single_bg));
        painter.text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            "单选题",
            egui::FontId::proportional(11.0),
            parse_color(&theme.badge_single_text),
        );

        let progress_y = 42.0;
        let progress_bg_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(18.0, progress_y),
            egui::vec2(240.0, 6.0),
        );
        painter.rect_filled(progress_bg_rect, 3.0, parse_color(&theme.progress_bg));
        let progress_fill_rect = egui::Rect::from_min_size(
            progress_bg_rect.min,
            egui::vec2(160.0, 6.0),
        );
        painter.rect_filled(progress_fill_rect, 3.0, parse_color(&theme.primary_dark));
        painter.text(
            progress_bg_rect.right_center() + egui::vec2(22.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "1/10",
            egui::FontId::proportional(12.0),
            parse_color(&theme.primary_dark),
        );

        let question_text = "1 + 1 = ?";
        painter.text(
            rect.min + egui::vec2(18.0, 60.0),
            egui::Align2::LEFT_TOP,
            question_text,
            egui::FontId::proportional(15.0),
            parse_color(&theme.text),
        );

        let option_y_start = 82.0;
        let options = ["A. 选项一", "B. 选项二 (选中)", "C. 选项三", "D. 选项四"];
        for (i, opt) in options.iter().enumerate() {
            let opt_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(18.0, option_y_start + i as f32 * 22.0),
                egui::vec2(264.0, 19.0),
            );
            if i == 1 {
                painter.rect_filled(opt_rect, 6.0, parse_color(&theme.highlight_bg));
                painter.rect_stroke(opt_rect, 6.0, Stroke::new(1.5, parse_color(&theme.highlight_border)));
                let letter_rect = egui::Rect::from_min_size(
                    opt_rect.min + egui::vec2(4.0, 2.0),
                    egui::vec2(16.0, 15.0),
                );
                painter.rect_filled(letter_rect, 8.0, parse_color(&theme.primary_mid));
                painter.text(
                    letter_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &opt[0..1],
                    egui::FontId::proportional(9.0),
                    Color32::WHITE,
                );
                painter.text(
                    opt_rect.min + egui::vec2(24.0, 3.0),
                    egui::Align2::LEFT_TOP,
                    &opt[2..],
                    egui::FontId::proportional(11.0),
                    parse_color(&theme.highlight_text),
                );
            } else {
                painter.rect_filled(opt_rect, 6.0, Color32::WHITE);
                painter.rect_stroke(opt_rect, 6.0, Stroke::new(1.0, parse_color(&theme.option_border)));
                let letter_rect = egui::Rect::from_min_size(
                    opt_rect.min + egui::vec2(4.0, 2.0),
                    egui::vec2(16.0, 15.0),
                );
                painter.rect_filled(letter_rect, 8.0, parse_color(&theme.option_letter_bg));
                painter.text(
                    letter_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &opt[0..1],
                    egui::FontId::proportional(9.0),
                    parse_color(&theme.primary_dark),
                );
                painter.text(
                    opt_rect.min + egui::vec2(24.0, 3.0),
                    egui::Align2::LEFT_TOP,
                    &opt[2..],
                    egui::FontId::proportional(11.0),
                    parse_color(&theme.text),
                );
            }
        }

        let btn_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(100.0, 158.0),
            egui::vec2(100.0, 24.0),
        );
        painter.rect_filled(btn_rect, 12.0, parse_color(&theme.btn_submit_bg));
        painter.text(
            btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "提交本题",
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        // 导入导出按钮
        let export_text = "导出主题";
        let import_text = "导入主题";
        let font = egui::FontId::proportional(13.0);
        let export_width = ui.fonts(|f| f.layout(export_text.to_string(), font.clone(), Color32::WHITE, f32::INFINITY).size().x) + 20.0;
        let import_width = ui.fonts(|f| f.layout(import_text.to_string(), font.clone(), Color32::WHITE, f32::INFINITY).size().x) + 20.0;
        let button_spacing = 16.0;
        let total_width = export_width + button_spacing + import_width;
        let start_x = rect.center().x - total_width / 2.0;
        let button_rect = egui::Rect::from_min_size(
            egui::pos2(start_x, rect.bottom() + 8.0),
            egui::vec2(total_width, 28.0),
        );
        ui.allocate_ui_at_rect(button_rect, |ui| {
            ui.horizontal(|ui| {
                ui.set_height(28.0);
                if ui.add_sized([export_width, 24.0], egui::Button::new(export_text)).clicked() {
                    let json = theme.to_json().unwrap_or_default();
                    if let Some(path) = FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .set_file_name(format!("{}.json", theme.name))
                        .save_file()
                    {
                        let _ = std::fs::write(path, json);
                    }
                }
                ui.add_space(button_spacing);
                if ui.add_sized([import_width, 24.0], egui::Button::new(import_text)).clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("主题文件", &["json"])
                        .set_directory(std::env::current_dir().unwrap_or_default())
                        .pick_file()
                    {
                        *pending_import = Some(path.to_string_lossy().to_string());
                    }
                }
            });
        });
    } else {
        ui.label("无预览");
    }
}