use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;
    let task_count = app.tasks.len();
    let mut idx_to_remove = None;

    // 提前获取所有需要的翻译字符串
    let file_prefix = app.tr("file_prefix").to_string();
    let remove_btn = app.tr("remove_btn").to_string();
    let dir_label = app.tr("dir_label").to_string();
    let export_dir_btn = app.tr("export_dir_btn").to_string();
    let file_name_label = app.tr("file_name_label").to_string();
    let page_title_label = app.tr("page_title_label").to_string();
    let filename_add_time = app.tr("filename_add_time").to_string();
    let display_time = app.tr("display_time").to_string();
    let use_current_time = app.tr("use_current_time").to_string();
    let time_input_label = app.tr("time_input_label").to_string();
    let theme_label = app.tr("theme_label").to_string();
    let follow_global = app.tr("follow_global").to_string();

    let button_width = 60.0;
    let right_margin = 6.0;
    let template_names: Vec<String> = app.templates.iter().map(|t| t.name().to_string()).collect();
    let label_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };

    for (i, task) in app.tasks.iter_mut().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}{}", file_prefix, task.input_path.file_name().unwrap().to_string_lossy()))
                        .size(14.0)
                        .color(label_color)
                        .strong(),
                );
                let spring = ui.available_width() - button_width - right_margin;
                if spring > 0.0 {
                    ui.add_space(spring);
                }
                if ui.add(egui::Button::new(&remove_btn).min_size(egui::vec2(button_width, 0.0))).clicked() {
                    idx_to_remove = Some(i);
                }
            });

            let (mut dir, mut filename) = {
                let p = std::path::Path::new(&task.output_path);
                let d = p.parent().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                let f = p.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                (d, f)
            };

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&dir_label).color(label_color));
                let edit_width = (ui.available_width() - button_width - right_margin - 30.0).max(60.0);
                if ui.add(egui::TextEdit::singleline(&mut dir).desired_width(edit_width)).changed() {
                    let new_path = std::path::Path::new(&dir).join(format!("{}.html", filename));
                    task.output_path = new_path.to_string_lossy().to_string();
                }
                ui.add_space(4.0);
                if ui.add(egui::Button::new(&export_dir_btn).min_size(egui::vec2(button_width, 0.0))).clicked() {
                    if let Some(folder) = FileDialog::new()
                        .set_directory(&dir)
                        .pick_folder()
                    {
                        let new_dir = folder.to_string_lossy().to_string();
                        dir = new_dir.clone();
                        let new_path = std::path::Path::new(&new_dir).join(format!("{}.html", filename));
                        task.output_path = new_path.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&file_name_label).color(label_color));
                if ui.add(egui::TextEdit::singleline(&mut filename).desired_width(ui.available_width())).changed() {
                    let new_path = std::path::Path::new(&dir).join(format!("{}.html", filename));
                    task.output_path = new_path.to_string_lossy().to_string();
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&page_title_label).color(label_color));
                ui.text_edit_singleline(&mut task.page_title);
            });

            // 时间控件边框逻辑保持不变
            let original_inactive_stroke = ui.style().visuals.widgets.inactive.bg_stroke;
            let original_hovered_stroke = ui.style().visuals.widgets.hovered.bg_stroke;
            let original_active_stroke = ui.style().visuals.widgets.active.bg_stroke;
            let new_stroke = egui::Stroke::new(1.5, Color32::from_rgb(80, 80, 120));
            {
                let style = ui.style_mut();
                style.visuals.widgets.inactive.bg_stroke = new_stroke;
                style.visuals.widgets.hovered.bg_stroke = new_stroke;
                style.visuals.widgets.active.bg_stroke = new_stroke;
            }

            ui.vertical(|ui| {
                ui.checkbox(&mut task.filename_add_time, &filename_add_time);
                ui.checkbox(&mut task.display_time, &display_time);
                if task.display_time {
                    ui.indent("time_options", |ui| {
                        ui.checkbox(&mut task.use_current_time, &use_current_time);
                        if !task.use_current_time {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&time_input_label).color(label_color));
                                {
                                    let style = ui.style_mut();
                                    style.visuals.widgets.inactive.bg_stroke = original_inactive_stroke;
                                    style.visuals.widgets.hovered.bg_stroke = original_hovered_stroke;
                                    style.visuals.widgets.active.bg_stroke = original_active_stroke;
                                }
                                ui.text_edit_singleline(&mut task.custom_time);
                                {
                                    let style = ui.style_mut();
                                    style.visuals.widgets.inactive.bg_stroke = new_stroke;
                                    style.visuals.widgets.hovered.bg_stroke = new_stroke;
                                    style.visuals.widgets.active.bg_stroke = new_stroke;
                                }
                            });
                        }
                    });
                }
            });

            {
                let style = ui.style_mut();
                style.visuals.widgets.inactive.bg_stroke = original_inactive_stroke;
                style.visuals.widgets.hovered.bg_stroke = original_hovered_stroke;
                style.visuals.widgets.active.bg_stroke = original_active_stroke;
            }

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&theme_label).color(label_color));
                let local_sel = task.selected_template;
                let display_text = match local_sel {
                    Some(idx) => app.templates[idx].name().to_string(),
                    None => follow_global.clone(),
                };
                egui::ComboBox::from_id_source(format!("task_template_{}", i))
                    .width(ui.available_width() - 60.0)
                    .selected_text(display_text)
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(local_sel.is_none(), &follow_global).clicked() {
                            task.selected_template = None;
                        }
                        for (idx, name) in template_names.iter().enumerate() {
                            if ui.selectable_label(local_sel == Some(idx), name.clone()).clicked() {
                                task.selected_template = Some(idx);
                            }
                        }
                    });
            });

            if !task.status.is_empty() {
                let color = if task.status.contains("[OK]") {
                    if dark { Color32::from_rgb(100, 200, 100) } else { Color32::from_rgb(60, 150, 60) }
                } else if task.status.contains("[FAIL]") {
                    if dark { Color32::from_rgb(255, 100, 100) } else { Color32::from_rgb(200, 80, 80) }
                } else {
                    if dark { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(100, 100, 100) }
                };
                ui.label(egui::RichText::new(&task.status).size(13.0).color(color));
            }
        });

        if i < task_count - 1 {
            ui.add_space(5.0);
        }
    }

    if let Some(idx) = idx_to_remove {
        app.tasks.remove(idx);
    }
}