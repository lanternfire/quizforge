use std::path::{Path, PathBuf};
use std::fs;
use crate::parser::parse_quiz_file;

pub struct QuizTask {
    pub input_path: PathBuf,
    pub output_path: String,
    pub page_title: String,
    pub status: String,
    pub completed: bool,
    pub selected_template: Option<usize>,
    pub display_time: bool,
    pub use_current_time: bool,
    pub custom_time: String,
    pub filename_add_time: bool,
}

impl QuizTask {
    pub fn from_input(path: PathBuf) -> Self {
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let parent = path.parent().unwrap_or(Path::new("."));
        let default_output = parent.join(format!("{}.html", stem)).to_string_lossy().to_string();
        let mut task = QuizTask {
            input_path: path.clone(),
            output_path: default_output,
            page_title: String::new(),
            status: String::new(),
            completed: false,
            selected_template: None,
            display_time: false,
            use_current_time: true,
            custom_time: String::new(),
            filename_add_time: false,
        };
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok((title_opt, _)) = parse_quiz_file(&content) {
                task.page_title = title_opt.unwrap_or_default();
            }
        }
        task
    }
}