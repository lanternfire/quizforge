use eframe::egui;
use egui::Color32;
use rfd::FileDialog;
use super::super::App;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;
    let task_count = app.tasks.len();
    let mut idx_to_remove = None;

    let button_width = 60.0;
    let right_margin = 6.0;
    let template_names: Vec<String> = app.templates.iter().map(|t| t.name().to_string()).collect();
    let label_color = if dark { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(44, 62, 80) };

    for (i, task) in app.tasks.iter_mut().enumerate() {
        ui.group(|ui| {
            // 文件名 + 移除按钮
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("文件: {}", task.input_path.file_name().unwrap().to_string_lossy()))
                        .size(14.0)
                        .color(label_color)
                        .strong(),
                );
                let spring = ui.available_width() - button_width - right_margin;
                if spring > 0.0 {
                    ui.add_space(spring);
                }
                if ui.add(egui::Button::new("移除").min_size(egui::vec2(button_width, 0.0))).clicked() {
                    idx_to_remove = Some(i);
                }
            });

            let (mut dir, mut filename) = {
                let p = std::path::Path::new(&task.output_path);
                let d = p.parent().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                let f = p.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                (d, f)
            };

            // 输出目录 + 导出目录按钮
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("输出目录：").color(label_color));
                let edit_width = (ui.available_width() - button_width - right_margin - 30.0).max(60.0);
                if ui.add(egui::TextEdit::singleline(&mut dir).desired_width(edit_width)).changed() {
                    let new_path = std::path::Path::new(&dir).join(format!("{}.html", filename));
                    task.output_path = new_path.to_string_lossy().to_string();
                }
                ui.add_space(4.0);
                if ui.add(egui::Button::new("导出目录").min_size(egui::vec2(button_width, 0.0))).clicked() {
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

            // 文件名
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("文件名：").color(label_color));
                if ui.add(egui::TextEdit::singleline(&mut filename).desired_width(ui.available_width())).changed() {
                    let new_path = std::path::Path::new(&dir).join(format!("{}.html", filename));
                    task.output_path = new_path.to_string_lossy().to_string();
                }
            });

            // 页面标题
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("页面标题：").color(label_color));
                ui.text_edit_singleline(&mut task.page_title);
            });

            // 时间控件
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
                ui.checkbox(&mut task.filename_add_time, "文件名加时间");
                ui.checkbox(&mut task.display_time, "显示生成时间");
                if task.display_time {
                    ui.indent("time_options", |ui| {
                        ui.checkbox(&mut task.use_current_time, "使用当前时间");
                        if !task.use_current_time {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("时间：").color(label_color));
                                // 恢复原始边框
                                {
                                    let style = ui.style_mut();
                                    style.visuals.widgets.inactive.bg_stroke = original_inactive_stroke;
                                    style.visuals.widgets.hovered.bg_stroke = original_hovered_stroke;
                                    style.visuals.widgets.active.bg_stroke = original_active_stroke;
                                }
                                ui.text_edit_singleline(&mut task.custom_time);
                                // 恢复增强边框
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

            // 恢复原始边框
            {
                let style = ui.style_mut();
                style.visuals.widgets.inactive.bg_stroke = original_inactive_stroke;
                style.visuals.widgets.hovered.bg_stroke = original_hovered_stroke;
                style.visuals.widgets.active.bg_stroke = original_active_stroke;
            }

            // 主题选择
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("主题：").color(label_color));
                let local_sel = task.selected_template;
                let display_text = match local_sel {
                    Some(idx) => app.templates[idx].name().to_string(),
                    None => "跟随全局设置".to_string(),
                };
                egui::ComboBox::from_id_source(format!("task_template_{}", i))
                    .width(ui.available_width() - 60.0)
                    .selected_text(display_text)
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(local_sel.is_none(), "跟随全局设置").clicked() {
                            task.selected_template = None;
                        }
                        for (idx, name) in template_names.iter().enumerate() {
                            if ui.selectable_label(local_sel == Some(idx), name.clone()).clicked() {
                                task.selected_template = Some(idx);
                            }
                        }
                    });
            });

            // 状态信息
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