use eframe::egui;
use egui::Color32;
use std::fs;
use chrono::Local;
use crate::parser::parse_quiz_file;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;

    // 提前获取翻译
    let generate_btn_text = app.tr("generate_btn").to_string();
    let gen_fail_read = app.tr("gen_fail_read").to_string();
    let gen_fail_parse = app.tr("gen_fail_parse").to_string();
    let gen_success = app.tr("gen_success").to_string();
    let gen_fail_write = app.tr("gen_fail_write").to_string();

    let gen_button = egui::Button::new(&generate_btn_text)
        .fill(if dark { Color32::from_rgb(50, 130, 80) } else { Color32::from_rgb(0xC0, 0xC0, 0xFF) })
        .rounding(10.0)
        .min_size(egui::vec2(ui.available_width(), 42.0));

    if ui.add(gen_button).clicked() {
        for task in &mut app.tasks {
            let content = match fs::read_to_string(&task.input_path) {
                Ok(c) => c,
                Err(e) => {
                    task.status = format!("[FAIL] {}: {}", gen_fail_read, e);
                    continue;
                }
            };
            let (title_from_file, questions) = match parse_quiz_file(&content) {
                Ok(r) => r,
                Err(e) => {
                    task.status = format!("[FAIL] {}: {}", gen_fail_parse, e);
                    continue;
                }
            };
            let page_title = if task.page_title.trim().is_empty() {
                title_from_file.unwrap_or_else(|| "在线练习".into())
            } else {
                task.page_title.clone()
            };
            let json = serde_json::to_string(&questions).unwrap();

            let generate_time_html = if task.display_time {
                let time_str = if task.use_current_time {
                    Local::now().format("%Y-%m-%d").to_string()
                } else {
                    task.custom_time.clone()
                };
                format!("<div class=\"generate-time\">生成时间：{}</div>", time_str)
            } else {
                String::new()
            };

            let output_path = if task.filename_add_time {
                let p = std::path::Path::new(&task.output_path);
                let parent = p.parent().unwrap_or_else(|| std::path::Path::new("."));
                let stem = p.file_stem().unwrap().to_string_lossy();
                let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_else(|| "html".to_string());
                let time_prefix = Local::now().format("%Y%m%d").to_string();
                let new_name = format!("{}_{}.{}", time_prefix, stem, ext);
                parent.join(new_name).to_string_lossy().to_string()
            } else {
                task.output_path.clone()
            };

            let template_idx = task.selected_template.unwrap_or(app.selected_template_idx);
            let template = &app.templates[template_idx];
            let html = template.generate_html(&page_title, &json, &generate_time_html);

            match fs::write(&output_path, html) {
                Ok(_) => {
                    task.status = format!("[OK] {} -> {}", gen_success, output_path);
                    task.completed = true;
                }
                Err(e) => task.status = format!("[FAIL] {}: {}", gen_fail_write, e),
            }
        }
    }
}