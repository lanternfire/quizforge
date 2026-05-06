mod localization;
mod parser;
mod templates;
mod theme;

use localization::Locale;
use parser::Question;
use std::sync::Mutex;
use tauri::State;
use theme::Theme;

/// Application state holding loaded themes and languages
pub struct AppState {
    pub themes: Mutex<Vec<Theme>>,
}

/// Parse a quiz file content and return the questions JSON
#[tauri::command]
fn parse_file(content: String) -> Result<(Option<String>, Vec<Question>), String> {
    parser::parse_quiz_file(&content).map_err(|e| e.to_string())
}

/// Get all available themes
#[tauri::command]
fn list_themes(state: State<AppState>) -> Vec<Theme> {
    state.themes.lock().unwrap().clone()
}

/// Generate HTML from questions and theme
#[tauri::command]
fn generate_html(
    title: String,
    questions_json: String,
    generate_time: String,
    locale_json: String,
    theme_idx: usize,
    state: State<AppState>,
) -> Result<String, String> {
    let themes = state.themes.lock().unwrap();
    let theme = themes.get(theme_idx).ok_or("主题索引无效")?;
    let locale = Locale::from_json(&locale_json);
    let html = templates::generate_html(&title, &questions_json, &generate_time, &locale, theme);
    Ok(html)
}

/// Import a theme from JSON file path
#[tauri::command]
fn import_theme(path: String, state: State<AppState>) -> Result<usize, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    let theme = Theme::from_json(&content).map_err(|e| format!("解析主题失败: {}", e))?;
    let mut themes = state.themes.lock().unwrap();
    themes.push(theme);
    Ok(themes.len() - 1)
}

/// Export a theme to JSON string
#[tauri::command]
fn export_theme(theme_idx: usize, state: State<AppState>) -> Result<String, String> {
    let themes = state.themes.lock().unwrap();
    let theme = themes.get(theme_idx).ok_or("主题索引无效")?;
    theme.to_json().map_err(|e| e.to_string())
}

/// Get current time string
#[tauri::command]
fn get_current_time() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Read file content for preview
#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// Write generated HTML to file
#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, &content).map_err(|e| e.to_string())
}

/// Load all built-in themes
fn load_builtin_themes() -> Vec<Theme> {
    let theme_jsons = [
        include_str!("../assets/themes/default.json"),
        include_str!("../assets/themes/orange.json"),
        include_str!("../assets/themes/green.json"),
        include_str!("../assets/themes/red.json"),
        include_str!("../assets/themes/pink.json"),
        include_str!("../assets/themes/yellow.json"),
        include_str!("../assets/themes/blue.json"),
        include_str!("../assets/themes/purple.json"),
        include_str!("../assets/themes/gray.json"),
    ];

    theme_jsons
        .iter()
        .filter_map(|json| Theme::from_json(json).ok())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let themes = load_builtin_themes();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            themes: Mutex::new(themes),
        })
        .invoke_handler(tauri::generate_handler![
            parse_file,
            list_themes,
            generate_html,
            import_theme,
            export_theme,
            get_current_time,
            read_file_content,
            write_file,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
