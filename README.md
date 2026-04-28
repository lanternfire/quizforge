# QuizForge (题匠)

A desktop tool to batch convert specially formatted `.txt` quiz files into self-contained interactive web pages (`.html`). Supports single choice, multiple choice, fill-in-the-blank (multiple blanks), and true/false questions. The generated pages work offline in any browser.

> :cn: [中文版 (Chinese)](/README_zh.md)

## Features

- Two-tab interface: main workspace and help center
- 9 built-in themes (default, orange, green, red, pink, yellow, blue, purple, gray) with expandable support
- Dark / light mode toggle, all controls adapt automatically
- Per-file theme assignment or follow global theme
- One-click "Apply to All" to reset all tasks to global theme
- Custom output directory and filename (suffix `.html` added automatically)
- Editable page title; falls back to the title in the quiz file, then to "在线练习"
- Question types: single choice, multiple choice, fill-in-the-blank, true/false
- Fill-in-the-blank supports multiple blanks (separated by semicolons)
- Generated HTML is self-contained, mobile-friendly, with question-by-question submission, scoring, and keyboard shortcuts
- `#` single-line comments in quiz files
- Chinese/English comma and semicolon compatibility for answers
- Drag and drop `.txt` files to import
- Timestamp options: add date to filename, display generation time on web page
- Theme colors stored in JSON under `assets/themes/`, compiled into the exe; import/export themes via JSON
- UI localization (Chinese / English), language files embedded in the exe
- Quiz file format supports both Chinese and English keywords (e.g., `题型：` vs `Type:`, `题目：` vs `Question:`)

## Tech Stack

- Rust 2021
- egui 0.27 + eframe 0.27
- rfd 0.14
- serde / serde_json
- anyhow
- image 0.24
- chrono

## Build & Run

### Requirements

- Rust toolchain (rustc, cargo)
- On Windows, installing `C:/Windows/Fonts/simhei.ttf` is recommended for Chinese text (falls back if missing)

### Release build

```bash
cargo build --release
```

The executable will be `target/release/quizforge.exe`. Distribute only the exe — all themes and languages are embedded.

### Development

```bash
cargo run
```

## Usage

### Prepare Quiz Files

Quiz files are plain `.txt`, UTF-8 recommended. Both **Chinese** and **English** formats are supported.

#### Chinese format (default)

```
# 这是一行注释
标题：我的题库

---
题型：单选
题目：35 + 28 = ?
选项：53|63|73|62
答案：1
```

#### English format (v1.1+)

```
# This is a comment
Title: My Quiz

---
Type: Single
Question: What is 1+1?
Options: 1|2|3|4
Answer: 2
```

**Rules**

- First block can optionally contain `标题：xxx` or `Title: xxx`, otherwise the file name is used.
- Questions are separated by `---` (three dashes).
- Each question must include: type, question, options (for choice types), and answer.
- `#` starts a comment.
- **Answer indices start from 1** (internally converted to 0).
- Single choice: a single number.
- Multiple choice: comma-separated numbers (Chinese/English comma both OK).
- Fill-in-the-blank: single blank — separate multiple acceptable answers with `|`. Multiple blanks — separate blanks with `;` (or `；`), each blank uses `|` for alternatives.
- True/false: `正确`/`错误` or `true`/`false` (also `yes`/`no`, `对`/`错`).

### Workflow

1. Send PDF/Word/image quizzes to AI (e.g., ChatGPT) to convert to standard txt.
2. Import the txt file (dialog or drag-and-drop).
3. Choose a template and click "Generate" / "批量生成 HTML 网页".

The right-side preview shows the selected theme; export/import buttons are below it.

### Switching Language

Use the dropdown menu (中文/English) in the top bar to switch the UI language instantly. All labels, buttons, help content, and preview text will update.

## Dark Mode

Click the "Dark" / "深色模式" button in the top bar to toggle dark/light mode.

## Theme Extension

Preset themes are stored in `assets/themes/` as JSON files. To add a new theme, create a new JSON and load it in `all_templates()` in `src/templates/mod.rs`. You can also import a `.json` theme via the "Import" button in the preview area.

## FAQ

**Does the generated page work on mobile?**  
Yes. The template uses responsive design.

**Is the fill-in-the-blank answer case-sensitive?**  
Answer matching is exact (case and full/half-width characters matter). Please match the expected answer exactly.

**How to set up multiple blanks?**  
Use `;` or `；` to separate blanks; each blank's answers are separated by `|`.

**Why does the program work as a single exe?**  
Themes and language files are embedded at compile time.

**How to use AI to generate txt files?**  
Open the "Help" tab, copy the AI prompt template, and send it to the AI along with your quiz materials.

**How to share or backup themes?**  
Use the "Export" button below the preview to save a theme as `.json`; use "Import" to load one.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file.