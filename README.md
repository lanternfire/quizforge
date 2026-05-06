# QuizForge v2

> Convert .txt quiz files into standalone interactive HTML quiz pages. No server needed — open the generated .html in any browser and start answering.

## Features

- **4 question types**: Single choice, Multiple choice, Fill-in-the-blank (multi-blank), Judgment
- **9 built-in themes**: Blue, Orange, Green, Red, Pink, Yellow, Purple, Gray — import/export custom themes
- **Dark/Light mode**: One-click toggle, all UI elements adapt automatically
- **Bilingual UI**: Switch between Chinese and English instantly
- **Live preview**: See how the quiz card looks before generating, theme changes update in real-time
- **Batch processing**: Import multiple .txt files, configure individually, generate all at once
- **Drag & drop**: Drag .txt files directly into the window
- **Timestamps**: Optionally prepend date to filenames or show generation time in the page footer
- **Self-contained HTML**: Generated .html includes all CSS/JS, zero external dependencies
- **Portable exe**: The app itself is a single .exe file — double-click to run, no installation

## Quick Start

1. Prepare a .txt quiz file following the format below
2. Open the app, click "Import" or drag files in
3. Pick a theme, click "Generate All", get .html files

## Quiz File Format

### English Format

```
# This line is a comment, will be ignored
Title: My Quiz

---
Type: Single
Question: What is 1+1?
Options: 1|2|3|4
Answer: 1

---
Type: Multiple
Question: Which are even numbers?
Options: 1|2|3|4
Answer: 2,4

---
Type: Fill
Question: What is 6×7?
Answer: 42|forty-two

---
Type: Fill (multi-blank)
Question: Two numbers, sum is 5, difference is 1
Answer: 2|two; 3|three

---
Type: Judgment
Question: The earth is round.
Answer: True
```

### Chinese Format (auto-detected)

```
标题：我的练习

---
题型：单选
题目：1+1=？
选项：1|2|3|4
答案：1

---
题型：多选
题目：哪些是偶数？
选项：1|2|3|4
答案：2,4

---
题型：填空
题目：6×7=？
答案：42|四十二

---
题型：判断
题目：地球是圆的
答案：正确
```

### Format Rules

| Rule | Description |
|------|-------------|
| File header | Optional `标题：xxx` or `Title: xxx`, defaults to filename |
| Separator | Questions separated by `---` (three dashes) |
| Comments | Lines starting with `#` are ignored |
| Keywords | 题型/Type, 题目/Question, 选项/Options, 答案/Answer |
| Answer index | Starts from 1 (1 = first option) |
| Multiple answers | Comma-separated (Chinese or English comma ok), e.g. `1,2,3` |
| Fill single blank | Pipe `|` separates acceptable answers: `42|forty-two` |
| Fill multi-blank | Semicolon `;` or `；` separates blanks, each blank uses `|` internally |
| Judgment values | `正确/错误`, `true/false`, `yes/no`, `对/错` |

## Build from Source

### Prerequisites

- Node.js 18+
- Rust 1.77+

### Build Command

```bash
build.bat
```

Output: `src-tauri/target/release/quizforge.exe` (standalone single file, ready to distribute)

### Development Mode

```bash
run_dev.bat
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend UI | React 18 + TypeScript |
| Build tool | Vite 5 |
| Desktop framework | Tauri 2 |
| Backend logic | Rust |
| Localization | JSON translation files |
