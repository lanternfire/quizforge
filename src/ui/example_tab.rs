use eframe::egui;
use crate::localization::Locale;

pub fn render(ui: &mut egui::Ui, dark_mode: bool, locale: &Locale) {
    let heading_color = if dark_mode {
        egui::Color32::from_rgb(130, 190, 230)
    } else {
        egui::Color32::from_rgb(46, 125, 166)
    };
    let text_color = if dark_mode {
        egui::Color32::from_rgb(240, 240, 240)
    } else {
        egui::Color32::from_rgb(0, 0, 0)
    };

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show(ui, |ui| {
            ui.heading(
                egui::RichText::new(locale.tr("tab_help"))
                    .color(heading_color)
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new(locale.tr("help_usage_title")).size(16.0).color(text_color).strong());
                ui.add_space(5.0);
                ui.label(locale.tr("help_usage_step1"));
                ui.label(locale.tr("help_usage_step2"));
                ui.label(locale.tr("help_usage_step3"));
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(locale.tr("help_ai_title")).size(16.0).color(text_color).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(
                            egui::Button::new(
                                egui::RichText::new(locale.tr("copy_prompt_btn")).color(egui::Color32::WHITE)
                            )
                            .fill(if dark_mode { egui::Color32::from_rgb(60, 130, 180) } else { egui::Color32::from_rgb(0xC0, 0xC0, 0xFF) })
                            .min_size(egui::vec2(100.0, 26.0))
                            .rounding(6.0)
                        ).clicked() {
                            let prompt = r#"请将以下题目（或图片中的题目）转换为标准 txt 题库文件，完全遵循以下格式规则：

【文件结构】
- 整个文件可包含一个可选的标题行：标题：你的题库名称（如：标题：数学练习一）
- 每道题之间用单独一行的三个减号 "---" 分隔
- 支持用 # 开头的行作为注释，会被自动忽略

【题目格式】
每道题必须包含：
题型：单选 / 多选 / 填空 / 判断
题目：题目的文字内容
选项：选项1|选项2|选项3 （仅单选和多选需要，用竖线分隔）
答案：根据题型提供不同格式（见下）

【答案格式详解】
- 单选题：填写正确选项的序号，从 1 开始计数。例如答案：2 表示第二个选项正确。
- 多选题：填写所有正确选项的序号，用英文逗号或中文逗号连接。例如：答案：1,3,4
- 填空题：
  - 如果只有一个空：多个可接受的答案用竖线 | 分隔。例如：答案：42|四十二
  - 如果有多个空：不同空的答案用分号（英文 ; 或中文 ；）分隔，每个空内部仍用竖线分隔多个可接受答案。例如：答案：2|二；3|三
- 判断题：答案只能是"正确"或"错误"（也可用 true/false）

【完整示例】
---
题型：单选
题目：35 + 28 = ?
选项：53|63|73|62
答案：1

---
题型：多选
题目：哪些算式等于24？
选项：8×3|30-6|20+4|4×6
答案：1,2,3,4

---
题型：填空
题目：6 × 7 = ?
答案：42|四十二

---
题型：填空
题目：两个数的和是5
答案：2|二；3|三

---
题型：判断
题目：81 - 36 = 45 正确吗？
答案：错误

【注意事项】
- 只输出题目文本，不要添加任何额外解释或对话。
- 选项的序号从 1 开始，务必使用正确的索引。
- 如果题目来自图片或文档，请严格按原意转换。
- 保持输出为纯文本，直接提供可用的 txt 内容。
"#;
                            ui.ctx().output_mut(|o| o.copied_text = prompt.to_string());
                        }
                    });
                });
                ui.add_space(6.0);

                let prompt_display = r#"请将以下题目（或图片中的题目）转换为标准 txt 题库文件，完全遵循以下格式规则：

【文件结构】
- 整个文件可包含一个可选的标题行：标题：你的题库名称（如：标题：数学练习一）
- 每道题之间用单独一行的三个减号 "---" 分隔
- 支持用 # 开头的行作为注释，会被自动忽略

【题目格式】
每道题必须包含：
题型：单选 / 多选 / 填空 / 判断
题目：题目的文字内容
选项：选项1|选项2|选项3 （仅单选和多选需要，用竖线分隔）
答案：根据题型提供不同格式（见下）

【答案格式详解】
- 单选题：填写正确选项的序号，从 1 开始计数。例如答案：2 表示第二个选项正确。
- 多选题：填写所有正确选项的序号，用英文逗号或中文逗号连接。例如：答案：1,3,4
- 填空题：
  - 如果只有一个空：多个可接受的答案用竖线 | 分隔。例如：答案：42|四十二
  - 如果有多个空：不同空的答案用分号（英文 ; 或中文 ；）分隔，每个空内部仍用竖线分隔多个可接受答案。例如：答案：2|二；3|三
- 判断题：答案只能是"正确"或"错误"（也可用 true/false）

【完整示例】
---
题型：单选
题目：35 + 28 = ?
选项：53|63|73|62
答案：1

---
题型：多选
题目：哪些算式等于24？
选项：8×3|30-6|20+4|4×6
答案：1,2,3,4

---
题型：填空
题目：6 × 7 = ?
答案：42|四十二

---
题型：填空
题目：两个数的和是5
答案：2|二；3|三

---
题型：判断
题目：81 - 36 = 45 正确吗？
答案：错误

【注意事项】
- 只输出题目文本，不要添加任何额外解释或对话。
- 选项的序号从 1 开始，务必使用正确的索引。
- 如果题目来自图片或文档，请严格按原意转换。
- 保持输出为纯文本，直接提供可用的 txt 内容。
"#;
                ui.add(
                    egui::TextEdit::multiline(&mut prompt_display.to_string())
                        .desired_width(ui.available_width())
                        .desired_rows(12)
                        .font(egui::TextStyle::Monospace)
                        .interactive(false)
                        .text_color(text_color)
                );
            });
        });
}