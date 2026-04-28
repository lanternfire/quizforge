mod tab_bar;
mod main_tab;
mod example_tab;

use eframe::egui;
use egui::{Color32, Rounding};
use std::sync::Arc;

use crate::task::QuizTask;
use crate::templates;
use crate::templates::theme::Theme;
use crate::templates::QuizTemplate;
use crate::localization::Locale;

pub struct App {
    pub tasks: Vec<QuizTask>,
    templates: Vec<Arc<dyn templates::Template>>,
    selected_template_idx: usize,
    active_tab: Tab,
    pub dark_mode: bool,
    pending_import_theme: Option<String>,
    locale: Locale,
    current_lang: String,
}

#[derive(PartialEq)]
enum Tab {
    Main,
    Example,
}

impl Default for App {
    fn default() -> Self {
        let templates = templates::all_templates();
        Self {
            tasks: Vec::new(),
            selected_template_idx: 0,
            templates,
            active_tab: Tab::Main,
            dark_mode: false,
            pending_import_theme: None,
            locale: Locale::load("zh_cn"),
            current_lang: "zh_cn".to_string(),
        }
    }
}

impl App {
    pub fn tr<'a>(&'a self, key: &'a str) -> &'a str {
        self.locale.tr(key)
    }
    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 主题设置（保持不变）
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
            ctx.style_mut(|style| {
                style.visuals.selection.bg_fill = Color32::from_rgb(74, 158, 197);
                style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
                style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
                style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
                style.visuals.widgets.active.rounding = Rounding::same(8.0);
                style.visuals.override_text_color = Some(Color32::from_rgb(240, 240, 240));
            });
        } else {
            ctx.set_visuals(egui::Visuals::light());
            ctx.style_mut(|style| {
                style.visuals.window_fill = Color32::from_rgb(0xF2, 0xF2, 0xFF);
                style.visuals.extreme_bg_color = Color32::from_rgb(0xF2, 0xF2, 0xFF);
                style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0xF0, 0xFF, 0xF8);
                style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(0xF8, 0xFF, 0xF0);
                style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(0xF0, 0xF8, 0xFF);
                style.visuals.widgets.active.bg_fill = Color32::from_rgb(0xC0, 0xC0, 0xFF);
                style.visuals.selection.bg_fill = Color32::from_rgb(0xC0, 0xC0, 0xFF);
                style.visuals.override_text_color = Some(Color32::from_rgb(44, 62, 80));
                style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
                style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
                style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
                style.visuals.widgets.active.rounding = Rounding::same(8.0);
            });
        }

        let bg_color = if self.dark_mode {
            Color32::from_rgb(30, 33, 40)
        } else {
            Color32::from_rgb(0xF6, 0xFD, 0xF6)
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::default()
                .fill(bg_color)
                .inner_margin(egui::vec2(20.0, 10.0)))
            .show(ctx, |ui| {
                tab_bar::render(ui, &mut self.active_tab, &mut self.dark_mode, &mut self.current_lang, &mut self.locale);
                match self.active_tab {
                    Tab::Main => main_tab::render(ui, self),
                    Tab::Example => example_tab::render(ui, self.dark_mode, self.locale()),
                }

                if self.active_tab == Tab::Main {
                    let version_text = self.tr("version");
                    let font = egui::FontId::monospace(11.0);
                    let text_color = if self.dark_mode {
                        Color32::from_gray(180)
                    } else {
                        Color32::from_gray(130)
                    };
                    let margin = 10.0;
                    let rect = ui.max_rect();
                    let pos = egui::pos2(rect.right() - margin, rect.bottom() - margin);
                    ui.painter().text(
                        pos,
                        egui::Align2::RIGHT_BOTTOM,
                        version_text,
                        font,
                        text_color,
                    );
                }
            });

        // 处理待导入的主题
        if let Some(path) = self.pending_import_theme.take() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(theme) = Theme::from_json(&content) {
                    let template = Arc::new(QuizTemplate::new(theme));
                    self.templates.push(template);
                    let new_idx = self.templates.len() - 1;
                    self.selected_template_idx = new_idx;
                }
            }
        }
    }
}