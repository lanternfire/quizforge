mod top_controls;
mod empty_state;
mod task_list;
mod generate_button;
mod preview;

use eframe::egui;
use egui::Color32;
use std::sync::Arc;
use super::{App, Tab};

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    let dark = app.dark_mode;
    let mut switch_to_help = false;
    let heading_color = if dark {
        Color32::from_rgb(130, 190, 230)
    } else {
        Color32::from_rgb(46, 125, 166)
    };
    let current_template: Arc<dyn crate::templates::Template> =
        Arc::clone(&app.templates[app.selected_template_idx]);

    let title_text = app.tr("app_title").to_string();

    ui.heading(
        egui::RichText::new(&title_text)
            .color(heading_color)
            .size(24.0)
            .strong(),
    );
    ui.add_space(15.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            top_controls::render(ui, app);
            ui.add_space(10.0);

            if app.tasks.is_empty() {
                empty_state::render(ui, app, &mut switch_to_help);
            } else {
                let button_height = 60.0;
                let available = ui.available_height();
                let scroll_height = (available - button_height).max(100.0);

                egui::ScrollArea::vertical()
                    .max_height(scroll_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        task_list::render(ui, app);
                    });

                ui.add_space(8.0);
                generate_button::render(ui, app);
            }
        });

        columns[1].vertical_centered(|ui| {
            preview::render(ui, current_template.as_ref(), dark, &mut app.pending_import_theme);
        });
    });

    if switch_to_help {
        app.active_tab = Tab::Example;
    }
}