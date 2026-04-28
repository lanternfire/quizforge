use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use crate::task::QuizTask;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;
    let label_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("全局模板：").size(15.0).color(label_color).strong());
        let mut selected_idx = app.selected_template_idx;
        let template_names: Vec<String> = app.templates.iter().map(|t| t.name().to_string()).collect();
        // 恢复原来的宽度：comboBox + 应用到全部 + 添加文件
        let combobox_width = ui.available_width() - 60.0 - 80.0 - 80.0;
        egui::ComboBox::from_id_source("global_template_selector")
            .width(combobox_width)
            .selected_text(template_names[selected_idx].clone())
            .show_ui(ui, |ui| {
                for (idx, name) in template_names.iter().enumerate() {
                    if ui.selectable_value(&mut selected_idx, idx, name.clone()).clicked() {
                        // 选中不同主题
                    }
                }
            });
        app.selected_template_idx = selected_idx;

        // 应用到全部
        if ui.add(egui::Button::new("应用到全部").min_size(egui::vec2(80.0, 0.0)).rounding(6.0)).clicked() {
            for task in &mut app.tasks {
                task.selected_template = None;
            }
        }

        // 添加文件
        if ui.add(egui::Button::new("添加文件").min_size(egui::vec2(80.0, 0.0)).rounding(6.0)).clicked() {
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