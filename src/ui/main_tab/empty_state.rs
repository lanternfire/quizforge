use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use crate::task::QuizTask;
use super::super::App;  // 只导入 App

pub fn render(ui: &mut egui::Ui, app: &mut App, switch_to_help: &mut bool) {
    let dark = app.dark_mode;
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        let text_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };
        ui.heading(
            egui::RichText::new("准备好生成答题网页了吗？")
                .size(18.0).color(text_color).strong()
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("将 PDF / Word / 图片题库交给 AI 整理成标准 txt，再导入本软件，一键生成 HTML。")
                .size(14.0)
                .color(if dark { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(80, 80, 80) })
        );
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new("导入 .txt 题库文件")
                .min_size(egui::vec2(180.0, 36.0))
            ).clicked() {
                if let Some(paths) = FileDialog::new()
                    .add_filter("题库文件", &["txt"])
                    .set_directory(std::env::current_dir().unwrap_or_default())
                    .pick_files()
                {
                    for path in paths {
                        if app.tasks.iter().any(|t| t.input_path == path) {
                            continue;
                        }
                        app.tasks.push(QuizTask::from_input(path));
                    }
                }
            }
            ui.add_space(12.0);
            if ui.add(egui::Button::new("查看如何制作 txt")
                .min_size(egui::vec2(180.0, 36.0))
            ).clicked() {
                *switch_to_help = true;
            }
        });
    });
}