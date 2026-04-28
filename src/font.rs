use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily, TextStyle};
use std::fs;

fn load_font(name: &str) -> Option<FontData> {
    let fonts_dir = std::path::Path::new("C:/Windows/Fonts");
    let font_path = fonts_dir.join(name);
    if let Ok(bytes) = fs::read(&font_path) {
        Some(FontData::from_owned(bytes))
    } else {
        None
    }
}

pub fn setup_fonts(ctx: &egui::Context) {
    ctx.set_pixels_per_point(1.0);

    let mut fonts = FontDefinitions::default();
    for family in fonts.families.values_mut() {
        family.clear();
    }

    if let Some(font_data) = load_font("simhei.ttf") {
        fonts.font_data.insert("SimHei".to_owned(), font_data);
        let font_name = "SimHei".to_owned();
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push(font_name.clone());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push(font_name);
    } else {
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("Arial".to_owned());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push("Consolas".to_owned());
    }

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, egui::FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Body, egui::FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Monospace, egui::FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Button, egui::FontId::new(15.0, FontFamily::Proportional)),
        (TextStyle::Small, egui::FontId::new(13.0, FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
    ctx.set_fonts(fonts);
}