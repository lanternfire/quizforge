#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod font;
mod parser;
mod task;
mod templates;
mod ui;
mod localization;   // 新增

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([820.0, 620.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "题匠",
        options,
        Box::new(|cc| {
            font::setup_fonts(&cc.egui_ctx);
            Box::new(ui::App::default())
        }),
    )
    .unwrap();
}