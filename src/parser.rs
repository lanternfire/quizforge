use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Question {
    #[serde(rename = "type")]
    pub qtype: String,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_bool: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_list: Option<Vec<usize>>,
}

pub fn parse_quiz_file(content: &str) -> anyhow::Result<(Option<String>, Vec<Question>)> {
    let content = content.replace("\r\n", "\n");
    let mut title = None;
    let mut questions = Vec::new();

    let blocks: Vec<&str> = content.split("\n---\n").collect();
    for (i, block) in blocks.iter().enumerate() {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        // 第一个块可能包含标题（支持中英文）
        if i == 0 {
            // 跳过空行和注释行，查找第一个有效的标题行
            let first_non_comment_non_empty = block.lines().find(|line| {
                let t = line.trim();
                !t.is_empty() && !t.starts_with('#')
            });
            if let Some(line) = first_non_comment_non_empty {
                let line = line.trim();
                // 中文标题：标题：...
                if let Some(v) = line.strip_prefix("标题：") {
                    title = Some(v.trim().to_string());
                }
                // 英文标题：Title: ...
                else if let Some(v) = line.strip_prefix("Title:") {
                    title = Some(v.trim().to_string());
                }
                // 如果整个块只有一行标题（且无题目），则跳过该块
                let non_comment_lines: Vec<_> = block.lines()
                    .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                    .collect();
                if non_comment_lines.len() == 1 && (line.starts_with("标题：") || line.starts_with("Title:")) {
                    continue;
                }
            }
        }

        let mut qtype = String::new();
        let mut question_text = String::new();
        let mut options_str = String::new();
        let mut answer_str = String::new();

        for line in block.lines() {
            let line = line.trim();
            if line.starts_with('#') {
                continue;
            }

            // 中文关键词
            if let Some(v) = line.strip_prefix("题型：") {
                qtype = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("题目：") {
                question_text = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("选项：") {
                options_str = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("答案：") {
                answer_str = v.trim().to_string();
            }
            // 英文关键词
            else if let Some(v) = line.strip_prefix("Type:") {
                let v = v.trim();
                match v {
                    "单选题" | "Single" | "single" | "SINGLE" => qtype = "单选".to_string(),
                    "多选题" | "Multiple" | "multiple" | "MULTIPLE" => qtype = "多选".to_string(),
                    "填空题" | "Fill" | "fill" | "FILL" | "Blank" | "blank" => qtype = "填空".to_string(),
                    "判断题" | "Judgment" | "judgment" | "True/False" | "true/false" | "TF" => qtype = "判断".to_string(),
                    _ => qtype = v.to_string(),
                }
            } else if let Some(v) = line.strip_prefix("Question:") {
                question_text = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("Options:") {
                options_str = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("Answer:") {
                answer_str = v.trim().to_string();
            }
        }

        anyhow::ensure!(!qtype.is_empty() && !question_text.is_empty(), "缺少题型或题目");

        let question = match qtype.as_str() {
            "单选" => {
                let options: Vec<String> = options_str.split('|').map(|s| s.trim().to_string()).collect();
                let normalized = answer_str.replace("，", ",");
                let correct: usize = normalized.trim().parse()?;
                anyhow::ensure!(correct >= 1, "单选题答案序号不能小于1（1表示第一个选项）");
                let correct = correct - 1;
                Question {
                    qtype: "single".into(),
                    question: question_text,
                    options: Some(options),
                    correct: Some(correct),
                    answers: None,
                    correct_bool: None,
                    correct_list: None,
                }
            }
            "多选" => {
                let options: Vec<String> = options_str.split('|').map(|s| s.trim().to_string()).collect();
                let normalized = answer_str.replace("，", ",");
                let raw_list: Vec<usize> = normalized
                    .split(',')
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;
                for idx in &raw_list {
                    anyhow::ensure!(*idx >= 1, "多选题答案序号不能小于1（1表示第一个选项）");
                }
                let correct_list: Vec<usize> = raw_list.into_iter().map(|x| x - 1).collect();
                Question {
                    qtype: "multiple".into(),
                    question: question_text,
                    options: Some(options),
                    correct_list: Some(correct_list),
                    correct: None,
                    answers: None,
                    correct_bool: None,
                }
            }
            "填空" => {
                let delimiter = if answer_str.contains('；') { '；' } else { ';' };
                let groups: Vec<&str> = answer_str.split(delimiter).collect();
                let mut answers: Vec<Vec<String>> = Vec::new();
                for g in groups {
                    let trimmed = g.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let parts: Vec<String> = trimmed.split('|').map(|s| s.trim().to_string()).collect();
                    answers.push(parts);
                }
                anyhow::ensure!(!answers.is_empty(), "填空题答案不能为空");
                Question {
                    qtype: "fill".into(),
                    question: question_text,
                    answers: Some(answers),
                    options: None,
                    correct: None,
                    correct_bool: None,
                    correct_list: None,
                }
            }
            "判断" => {
                let correct_bool = match answer_str.trim() {
                    "正确" | "true" | "True" | "TRUE" | "yes" | "Yes" | "YES" | "对" => true,
                    "错误" | "false" | "False" | "FALSE" | "no" | "No" | "NO" | "错" => false,
                    _ => anyhow::bail!("判断题答案应为 正确/错误 或 true/false"),
                };
                Question {
                    qtype: "judge".into(),
                    question: question_text,
                    correct_bool: Some(correct_bool),
                    options: None,
                    correct: None,
                    answers: None,
                    correct_list: None,
                }
            }
            _ => anyhow::bail!("不支持的题型：{qtype}"),
        };
        questions.push(question);
    }
    Ok((title, questions))
}