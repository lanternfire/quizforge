use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use crate::task::QuizTask;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;
    let label_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };

    // 提前获取翻译
    let global_template = app.tr("global_template").to_string();
    let apply_all = app.tr("apply_all").to_string();
    let add_file = app.tr("add_file").to_string();

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(&global_template).size(15.0).color(label_color).strong());
        let mut selected_idx = app.selected_template_idx;
        let template_names: Vec<String> = app.templates.iter().map(|t| t.name().to_string()).collect();
        let combobox_width = ui.available_width() - 60.0 - 80.0 - 80.0;
        egui::ComboBox::from_id_source("global_template_selector")
            .width(combobox_width)
            .selected_text(template_names[selected_idx].clone())
            .show_ui(ui, |ui| {
                for (idx, name) in template_names.iter().enumerate() {
                    if ui.selectable_value(&mut selected_idx, idx, name.clone()).clicked() {}
                }
            });
        app.selected_template_idx = selected_idx;

        if ui.add(egui::Button::new(&apply_all).min_size(egui::vec2(80.0, 0.0)).rounding(6.0)).clicked() {
            for task in &mut app.tasks {
                task.selected_template = None;
            }
        }

        if ui.add(egui::Button::new(&add_file).min_size(egui::vec2(80.0, 0.0)).rounding(6.0)).clicked() {
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
    });
}