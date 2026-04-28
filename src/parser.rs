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

        // 对第一个块特殊处理：提取标题（忽略注释行）
        let mut title_line_index = None;
        if i == 0 {
            let lines: Vec<&str> = block.lines().collect();
            // 查找第一个非注释非空行
            if let Some(pos) = lines.iter().position(|line| {
                let t = line.trim();
                !t.is_empty() && !t.starts_with('#')
            }) {
                let first_line = lines[pos].trim();
                if let Some(v) = first_line.strip_prefix("标题：") {
                    title = Some(v.trim().to_string());
                    title_line_index = Some(pos);
                    // 如果整个区块只有标题和注释，则直接跳过
                    let non_comment_count = lines.iter().filter(|line| {
                        let t = line.trim();
                        !t.is_empty() && !t.starts_with('#')
                    }).count();
                    if non_comment_count == 1 {
                        continue;
                    }
                }
            }
        }

        let mut qtype = String::new();
        let mut question_text = String::new();
        let mut options_str = String::new();
        let mut answer_str = String::new();

        for (line_idx, line) in block.lines().enumerate() {
            let line = line.trim();
            // 跳过已提取的标题行
            if title_line_index == Some(line_idx) {
                continue;
            }
            // 注释行跳过
            if line.starts_with('#') {
                continue;
            }

            if let Some(v) = line.strip_prefix("题型：") {
                qtype = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("题目：") {
                question_text = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("选项：") {
                options_str = v.trim().to_string();
            } else if let Some(v) = line.strip_prefix("答案：") {
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
                    "正确" | "true" | "True" => true,
                    "错误" | "false" | "False" => false,
                    _ => anyhow::bail!("判断题答案应为 正确/错误"),
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