use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use crate::task::QuizTask;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App, switch_to_help: &mut bool) {
    let dark = app.dark_mode;
    // 提前提取翻译
    let empty_title = app.tr("empty_title").to_string();
    let empty_desc = app.tr("empty_desc").to_string();
    let import_txt_btn = app.tr("import_txt_btn").to_string();
    let how_to_make_txt = app.tr("how_to_make_txt").to_string();

    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        let text_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };
        ui.heading(
            egui::RichText::new(&empty_title)
                .size(18.0).color(text_color).strong()
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(&empty_desc)
                .size(14.0)
                .color(if dark { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(80, 80, 80) })
        );
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(&import_txt_btn)
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
            if ui.add(egui::Button::new(&how_to_make_txt)
                .min_size(egui::vec2(180.0, 36.0))
            ).clicked() {
                *switch_to_help = true;
            }
        });
    });
}