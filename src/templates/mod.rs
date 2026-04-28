pub mod theme;

use std::sync::Arc;
use theme::Theme;

pub trait Template: Send + Sync {
    fn name(&self) -> &str;
    fn generate_html(&self, title: &str, questions_json: &str, generate_time: &str) -> String;
    fn theme_colors(&self) -> Option<&Theme> {
        None
    }
}

pub struct QuizTemplate {
    theme: Theme,
}

impl QuizTemplate {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

impl Template for QuizTemplate {
    fn name(&self) -> &str {
        &self.theme.name
    }

    fn theme_colors(&self) -> Option<&Theme> {
        Some(&self.theme)
    }

    fn generate_html(&self, title: &str, questions_json: &str, generate_time: &str) -> String {
        let tmpl = r###"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{PAGE_TITLE}}</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }

        body {
            background-color: {{bg}};
            background-image:
                radial-gradient(ellipse at 50% 20%, {{primary_light}} 0%, transparent 70%),
                radial-gradient(ellipse at 50% 80%, {{primary}} 0%, transparent 70%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: "SimSun", "宋体", "Songti SC", "NSimSun", serif;
            color: {{text}};
            padding: 20px;
            -webkit-tap-highlight-color: transparent;
            user-select: none;
        }

        .main-container {
            width: 100%;
            max-width: 680px;
            background: {{card_bg}};
            border-radius: 20px;
            box-shadow: {{shadow}};
            padding: 36px 32px 28px;
            border: 1px solid {{border}};
            position: relative;
            transition: box-shadow 0.2s;
        }

        .main-container:hover { box-shadow: {{shadow_hover}}; }

        .header { text-align: center; margin-bottom: 10px; }
        .header .icon { font-size: 38px; display: block; margin-bottom: 2px; }
        .header .title {
            font-size: 22px;
            font-weight: bold;
            color: {{primary_deep}};
            letter-spacing: 1px;
        }
        .header .subtitle {
            font-size: 13px;
            color: {{text_light}};
            margin-top: 2px;
            letter-spacing: 0.5px;
        }

        .generate-time {
            text-align: center;
            font-size: 13px;
            color: {{time_color}};
            margin-top: 4px;
        }

        .progress-wrap { display: flex; align-items: center; gap: 10px; margin: 16px 0 20px; }
        .progress-bar-outer { flex: 1; height: 8px; background: {{progress_bg}}; border-radius: 10px; overflow: hidden; }
        .progress-bar-inner {
            height: 100%;
            background: linear-gradient(90deg, {{progress_gradient_start}}, {{progress_gradient_end}});
            border-radius: 10px;
            transition: width 0.35s ease;
        }
        .progress-text {
            font-size: 14px;
            font-weight: 600;
            color: {{primary_dark}};
            min-width: 40px;
            text-align: right;
            letter-spacing: 0.5px;
        }

        .question-card {
            background: {{question_bg}};
            border: 1.5px solid {{question_border}};
            border-radius: 16px;
            padding: 24px 22px 20px;
            margin-bottom: 10px;
            min-height: 200px;
            transition: all 0.2s;
            position: relative;
        }
        .question-card.correct-flash {
            border-color: {{correct_border}};
            background: {{correct_bg}};
            box-shadow: 0 0 0 6px rgba(125, 206, 130, 0.08);
        }
        .question-card.wrong-flash {
            border-color: {{wrong_border}};
            background: {{wrong_bg}};
            box-shadow: 0 0 0 6px rgba(232, 139, 139, 0.07);
        }

        .q-meta { display: flex; align-items: center; gap: 10px; margin-bottom: 14px; flex-wrap: wrap; }
        .q-number {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 36px;
            height: 36px;
            border-radius: 50%;
            background: {{primary_light}};
            color: {{primary_deep}};
            font-weight: 700;
            font-size: 16px;
            flex-shrink: 0;
        }
        .q-type-badge {
            display: inline-block;
            font-size: 12px;
            padding: 4px 10px;
            border-radius: 20px;
            background: {{badge_bg}};
            color: {{primary_dark}};
            letter-spacing: 0.5px;
            font-weight: 500;
        }
        .q-type-badge.single { background: {{badge_single_bg}}; color: {{badge_single_text}}; }
        .q-type-badge.fill { background: {{badge_fill_bg}}; color: {{badge_fill_text}}; }
        .q-type-badge.judge { background: {{badge_judge_bg}}; color: {{badge_judge_text}}; }
        .q-type-badge.multiple { background: {{badge_multiple_bg}}; color: {{badge_multiple_text}}; }

        .q-text {
            font-size: 18px;
            font-weight: 600;
            color: {{text}};
            line-height: 1.7;
            margin-bottom: 16px;
            letter-spacing: 0.3px;
        }

        .options-area { display: flex; flex-direction: column; gap: 10px; }
        .option-btn {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 13px 16px;
            border: 2px solid {{option_border}};
            border-radius: 8px;
            background: #fff;
            cursor: pointer;
            font-size: 16px;
            color: {{text}};
            transition: all 0.2s;
            letter-spacing: 0.3px;
            position: relative;
            outline: none;
        }
        .option-btn:hover:not(:disabled) {
            border-color: {{option_hover_border}};
            background: {{option_hover_bg}};
            transform: translateY(-1px);
            box-shadow: 0 2px 8px rgba(150, 190, 210, 0.12);
        }
        .option-btn:active:not(:disabled) { transform: scale(0.985); }
        .option-btn .opt-letter {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 30px;
            height: 30px;
            border-radius: 50%;
            background: {{option_letter_bg}};
            font-weight: 700;
            font-size: 14px;
            color: {{primary_dark}};
            flex-shrink: 0;
            transition: all 0.2s;
        }
        .option-btn.selected {
            border-color: {{option_selected_border}} !important;
            background: {{option_selected_bg}} !important;
            box-shadow: 0 0 0 5px rgba(91, 174, 214, 0.1) !important;
            color: {{highlight_text}} !important;
            font-weight: 600;
        }
        .option-btn.selected .opt-letter {
            background: {{primary_mid}};
            color: #fff;
        }
        .option-btn.result-correct {
            border-color: {{option_correct_border}} !important;
            background: {{option_correct_bg}} !important;
            box-shadow: 0 0 0 5px rgba(125, 206, 130, 0.1) !important;
            color: #3a7d3e !important;
        }
        .option-btn.result-correct .opt-letter { background: #7dce82; color: #fff; }
        .option-btn.result-wrong-choice {
            border-color: {{option_wrong_border}} !important;
            background: {{option_wrong_bg}} !important;
            box-shadow: 0 0 0 5px rgba(232, 139, 139, 0.08) !important;
            color: #b54545 !important;
        }
        .option-btn.result-wrong-choice .opt-letter { background: #e88b8b; color: #fff; }
        .option-btn.result-missed {
            border-color: {{option_missed_border}} !important;
            background: {{option_missed_bg}} !important;
            box-shadow: 0 0 0 5px rgba(240, 200, 122, 0.12) !important;
        }
        .option-btn:disabled { cursor: default; opacity: 0.9; }

        .fill-inputs-wrap { display: flex; flex-direction: column; gap: 14px; }
        .fill-single {
            display: flex;
            align-items: center;
            gap: 10px;
            font-size: 16px;
            color: {{text}};
        }
        .fill-input {
            flex: 1;
            max-width: 320px;
            padding: 13px 18px;
            border: 2px solid {{fill_input_border}};
            border-radius: 8px;
            font-size: 18px;
            color: {{text}};
            background: #fff;
            outline: none;
            transition: all 0.2s;
            letter-spacing: 1px;
        }
        .fill-input:focus:not(:disabled) {
            border-color: {{fill_input_focus_border}};
            box-shadow: 0 0 0 6px {{fill_input_focus_shadow}};
            background: #fdfeff;
        }
        .fill-input.has-value {
            border-color: {{fill_input_has_value_border}};
            background: #fcfefe;
            box-shadow: 0 0 0 4px {{fill_input_has_value_shadow}};
        }
        .fill-input.result-correct {
            border-color: {{option_correct_border}} !important;
            background: {{option_correct_bg}} !important;
            box-shadow: 0 0 0 6px rgba(125, 206, 130, 0.1) !important;
            color: #3a7d3e !important;
        }
        .fill-input.result-wrong {
            border-color: {{option_wrong_border}} !important;
            background: {{option_wrong_bg}} !important;
            box-shadow: 0 0 0 6px rgba(232, 139, 139, 0.08) !important;
            color: #b54545 !important;
        }
        .fill-input:disabled { background: #f9fafb; cursor: default; opacity: 0.9; }
        .fill-correct-answers {
            font-size: 13px;
            color: {{fill_correct_text}};
            margin-top: 6px;
            font-weight: 500;
        }

        .judge-options { display: flex; gap: 14px; }
        .judge-btn {
            flex: 1;
            max-width: 140px;
            padding: 14px 20px;
            border: 2px solid {{judge_border}};
            border-radius: 8px;
            background: #fff;
            cursor: pointer;
            font-size: 17px;
            font-weight: 600;
            color: {{text}};
            transition: all 0.2s;
            text-align: center;
            letter-spacing: 0.5px;
            outline: none;
        }
        .judge-btn:hover:not(:disabled) {
            border-color: {{judge_hover_border}};
            background: #fbfdfe;
            transform: translateY(-1px);
            box-shadow: 0 2px 8px rgba(150, 190, 210, 0.12);
        }
        .judge-btn:active:not(:disabled) { transform: scale(0.96); }
        .judge-btn.selected {
            border-color: {{judge_selected_border}} !important;
            background: {{judge_selected_bg}} !important;
            box-shadow: 0 0 0 5px rgba(91, 174, 214, 0.1) !important;
            color: {{highlight_text}} !important;
        }
        .judge-btn.result-correct {
            border-color: {{judge_correct_border}} !important;
            background: {{judge_correct_bg}} !important;
            color: #3a7d3e !important;
            box-shadow: 0 0 0 5px rgba(125, 206, 130, 0.1) !important;
        }
        .judge-btn.result-wrong-choice {
            border-color: {{judge_wrong_border}} !important;
            background: {{judge_wrong_bg}} !important;
            color: #b54545 !important;
            box-shadow: 0 0 0 5px rgba(232, 139, 139, 0.08) !important;
        }
        .judge-btn:disabled { cursor: default; opacity: 0.9; }

        .feedback-icon {
            position: absolute;
            top: 14px;
            right: 16px;
            font-size: 28px;
            pointer-events: none;
            z-index: 2;
            animation: popIn 0.35s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        }
        @keyframes popIn {
            0% { transform: scale(0); opacity: 0; }
            70% { transform: scale(1.2); opacity: 1; }
            100% { transform: scale(1); opacity: 1; }
        }

        .btn-row { display: flex; align-items: center; justify-content: center; gap: 14px; margin-top: 18px; flex-wrap: wrap; }
        .btn {
            padding: 12px 26px;
            border-radius: 25px;
            font-size: 15px;
            font-weight: 600;
            cursor: pointer;
            border: none;
            letter-spacing: 0.5px;
            transition: all 0.2s;
            outline: none;
            white-space: nowrap;
            min-width: 85px;
        }
        .btn:active:not(:disabled) { transform: scale(0.95); }
        .btn:disabled { opacity: 0.5; cursor: not-allowed; filter: grayscale(15%); }

        .btn-prev { background: {{btn_prev_bg}}; color: #4a7d96; border: 1.5px solid #d0dde6; }
        .btn-prev:hover:not(:disabled) { background: {{btn_prev_hover}}; border-color: #bcccd8; }
        .btn-submit { background: {{btn_submit_bg}}; color: #fff; border: 1.5px solid transparent; box-shadow: 0 3px 12px rgba(74,158,197,0.3); letter-spacing: 1px; font-size: 16px; min-width: 95px; }
        .btn-submit:hover:not(:disabled) { background: {{btn_submit_hover}}; box-shadow: 0 5px 18px rgba(74,158,197,0.4); transform: translateY(-1px); }
        .btn-next { background: {{btn_next_bg}}; color: #4a7d96; border: 1.5px solid #d0dde6; }
        .btn-next:hover:not(:disabled) { background: {{btn_next_hover}}; border-color: #bcccd8; }
        .btn-reset { background: {{btn_reset_bg}}; color: #666; border: 1.5px solid #ddd; font-size: 14px; }
        .btn-reset:hover { background: {{btn_reset_hover}}; border-color: #ccc; }

        .score-bar {
            display: none;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-top: 16px;
            padding: 14px 20px;
            background: {{score_bg}};
            border-radius: 25px;
            border: 1.5px solid {{score_border}};
            font-size: 16px;
            font-weight: 600;
            color: {{primary_deep}};
            letter-spacing: 0.5px;
            text-align: center;
            flex-wrap: wrap;
        }
        .score-bar.show { display: flex; }
        .score-bar .score-num { font-size: 26px; font-weight: 800; color: {{score_num_color}}; }
        .score-bar .score-total { font-weight: 700; color: {{score_total_color}}; }
        .score-bar .score-emoji { font-size: 24px; }

        @media (max-width: 600px) {
            .main-container { padding: 24px 16px 20px; border-radius: 16px; }
            .q-text { font-size: 16px; }
            .option-btn { font-size: 15px; padding: 11px 13px; }
            .option-btn .opt-letter { width: 26px; height: 26px; font-size: 13px; }
            .btn { padding: 10px 18px; font-size: 14px; min-width: 65px; }
            .btn-submit { font-size: 15px; min-width: 75px; }
            .judge-btn { font-size: 15px; padding: 12px 16px; max-width: 120px; }
            .fill-input { font-size: 16px; max-width: 100%; padding: 11px 14px; }
            .btn-row { gap: 8px; }
            .header .title { font-size: 19px; }
            .feedback-icon { font-size: 24px; top: 10px; right: 12px; }
        }
    </style>
</head>
<body>
<div class="main-container" id="mainContainer">
    <div class="header">
        <span class="icon">  </span>
        <span class="title">{{PAGE_TITLE}}</span>
        <div class="subtitle" id="subtitle">共 <span id="totalQuestions">0</span> 题 · 逐题提交</div>
        <div class="generate-time">{{GENERATE_TIME}}</div>
    </div>

    <div class="progress-wrap">
        <div class="progress-bar-outer">
            <div class="progress-bar-inner" id="progressBar" style="width: 10%;"></div>
        </div>
        <span class="progress-text" id="progressText">1/10</span>
    </div>

    <div class="question-card" id="questionCard"></div>

    <div class="btn-row">
        <button class="btn btn-prev" id="btnPrev"> 上一题</button>
        <button class="btn btn-submit" id="btnSubmit"> 提交本题</button>
        <button class="btn btn-next" id="btnNext">下一题 </button>
    </div>

    <div class="score-bar" id="scoreBar">
        <span class="score-emoji" id="scoreEmoji"> </span>
        <span>得分：</span>
        <span class="score-num" id="scoreNum">0</span>
        <span>/</span>
        <span class="score-total" id="scoreTotal">10</span>
        <button class="btn btn-reset" id="btnReset"> 重新答题</button>
    </div>
</div>

<script>
    (function() {
        const questionsData = {{QUESTIONS_DATA}};

        document.getElementById('totalQuestions').textContent = questionsData.length;
        document.getElementById('scoreTotal').textContent = questionsData.length;

        const state = {
            currentIndex: 0,
            userAnswers: questionsData.map(q => {
                if (q.type === 'single') return null;
                if (q.type === 'fill') {
                    return new Array(q.answers ? q.answers.length : 1).fill('');
                }
                if (q.type === 'judge') return null;
                if (q.type === 'multiple') return new Set();
                return null;
            }),
            submittedFlags: new Array(questionsData.length).fill(false),
            scoreShown: false
        };

        const dom = {
            card: document.getElementById('questionCard'),
            btnPrev: document.getElementById('btnPrev'),
            btnSubmit: document.getElementById('btnSubmit'),
            btnNext: document.getElementById('btnNext'),
            btnReset: document.getElementById('btnReset'),
            scoreBar: document.getElementById('scoreBar'),
            scoreNum: document.getElementById('scoreNum'),
            scoreEmoji: document.getElementById('scoreEmoji'),
            progressBar: document.getElementById('progressBar'),
            progressText: document.getElementById('progressText')
        };

        function escapeHtml(str) {
            const div = document.createElement('div');
            div.textContent = str;
            return div.innerHTML;
        }

        function isAnswerCorrect(index) {
            const q = questionsData[index];
            const ua = state.userAnswers[index];
            if (q.type === 'single') return ua === q.correct;
            if (q.type === 'fill') {
                if (!ua || !(ua instanceof Array)) return false;
                if (ua.length !== q.answers.length) return false;
                return q.answers.every((group, i) => {
                    const userAnswer = (ua[i] || '').trim();
                    return group.some(a => a === userAnswer);
                });
            }
            if (q.type === 'judge') return ua === q.correct_bool;
            if (q.type === 'multiple') {
                if (!(ua instanceof Set)) return false;
                const correctSet = new Set(q.correct_list);
                if (ua.size !== correctSet.size) return false;
                for (const item of ua) if (!correctSet.has(item)) return false;
                return true;
            }
            return false;
        }

        function allSubmitted() {
            return state.submittedFlags.every(flag => flag === true);
        }

        function countCorrect() {
            let count = 0;
            for (let i = 0; i < questionsData.length; i++) {
                if (isAnswerCorrect(i)) count++;
            }
            return count;
        }

        function showScoreIfAllSubmitted() {
            if (!allSubmitted() || state.scoreShown) return;
            state.scoreShown = true;
            const correct = countCorrect();
            dom.scoreNum.textContent = correct;
            const total = questionsData.length;
            if (correct === total) dom.scoreEmoji.textContent = '[完美]';
            else if (correct >= total * 0.8) dom.scoreEmoji.textContent = '[优秀]';
            else if (correct >= total * 0.6) dom.scoreEmoji.textContent = '[良好]';
            else if (correct >= total * 0.4) dom.scoreEmoji.textContent = '[加油]';
            else dom.scoreEmoji.textContent = '[继续努力]';
            dom.scoreBar.classList.add('show');
            dom.btnReset.style.display = 'inline-block';
            dom.btnSubmit.disabled = true;
            dom.btnSubmit.textContent = '已全部完成';
            setTimeout(() => dom.scoreBar.scrollIntoView({ behavior: 'smooth', block: 'center' }), 200);
        }

        function hideScore() {
            state.scoreShown = false;
            dom.scoreBar.classList.remove('show');
            dom.btnReset.style.display = 'none';
            dom.scoreNum.textContent = '0';
            dom.scoreEmoji.textContent = '';
            dom.btnSubmit.disabled = false;
        }

        function render() {
            const index = state.currentIndex;
            const q = questionsData[index];
            const ua = state.userAnswers[index];
            const submitted = state.submittedFlags[index];
            let html = '';

            const typeLabels = { single: '单选题', fill: '填空题', judge: '判断题', multiple: '多选题' };
            const typeClassMap = { single: 'single', fill: 'fill', judge: 'judge', multiple: 'multiple' };
            html += `<div class="q-meta">
                <span class="q-number">${index + 1}</span>
                <span class="q-type-badge ${typeClassMap[q.type]}">${typeLabels[q.type]}</span>
            </div>`;
            html += `<div class="q-text">${q.question}</div>`;

            if (q.type === 'single') {
                html += `<div class="options-area">`;
                const letters = ['A','B','C','D'];
                q.options.forEach((opt, i) => {
                    let cls = 'option-btn';
                    if (!submitted && ua === i) cls += ' selected';
                    if (submitted) {
                        if (i === q.correct) cls += ' result-correct';
                        else if (ua === i && i !== q.correct) cls += ' result-wrong-choice';
                    }
                    html += `<button class="${cls}" data-opt="${i}" ${submitted ? 'disabled' : ''}>${letters[i]}. ${opt}</button>`;
                });
                html += `</div>`;
            } else if (q.type === 'fill') {
                const blankCount = q.answers.length;
                html += `<div class="fill-inputs-wrap">`;
                for (let bi = 0; bi < blankCount; bi++) {
                    let val = ua[bi] || '';
                    let inputCls = 'fill-input';
                    if (!submitted && val.trim() !== '') inputCls += ' has-value';
                    if (submitted) {
                        const groupCorrect = q.answers[bi].some(a => a === val.trim());
                        inputCls += groupCorrect ? ' result-correct' : ' result-wrong';
                    }
                    const label = blankCount > 1 ? `填空${bi+1}：` : '';
                    html += `<div class="fill-single">${label}<input type="text" class="${inputCls}" id="fillInput${index}_${bi}" value="${escapeHtml(val)}" 
                               placeholder="请输入答案" ${submitted ? 'disabled' : ''} autocomplete="off"></div>`;
                }
                html += `</div>`;
                if (submitted && !isAnswerCorrect(index)) {
                    const correctText = q.answers.map(group => group.join(' 或 ')).join(' ； ');
                    html += `<div class="fill-correct-answers">正确答案：${correctText}</div>`;
                }
            } else if (q.type === 'judge') {
                html += `<div class="judge-options">`;
                const opts = [{v:true, l:'正确'}, {v:false, l:'错误'}];
                opts.forEach(o => {
                    let cls = 'judge-btn';
                    if (!submitted && ua === o.v) cls += ' selected';
                    if (submitted) {
                        if (o.v === q.correct_bool) cls += ' result-correct';
                        else if (ua === o.v && ua !== q.correct_bool) cls += ' result-wrong-choice';
                    }
                    html += `<button class="${cls}" data-judge="${o.v}" ${submitted ? 'disabled' : ''}>${o.l}</button>`;
                });
                html += `</div>`;
            } else if (q.type === 'multiple') {
                html += `<div class="options-area">`;
                const letters = ['A','B','C','D'];
                const selectedSet = (ua instanceof Set) ? ua : new Set();
                q.options.forEach((opt, i) => {
                    let cls = 'option-btn';
                    if (!submitted && selectedSet.has(i)) cls += ' selected';
                    if (submitted) {
                        const isCorrectOpt = q.correct_list.includes(i);
                        const isSelected = selectedSet.has(i);
                        if (isCorrectOpt && isSelected) cls += ' result-correct';
                        else if (isCorrectOpt && !isSelected) cls += ' result-missed';
                        else if (!isCorrectOpt && isSelected) cls += ' result-wrong-choice';
                    }
                    html += `<button class="${cls}" data-opt="${i}" ${submitted ? 'disabled' : ''}>${letters[i]}. ${opt}</button>`;
                });
                html += `</div>`;
            }

            if (submitted) {
                const icon = isAnswerCorrect(index) ? '[对]' : '[错]';
                html += `<span class="feedback-icon">${icon}</span>`;
            }

            dom.card.innerHTML = html;

            dom.card.classList.remove('correct-flash', 'wrong-flash');
            if (submitted) {
                dom.card.classList.add(isAnswerCorrect(index) ? 'correct-flash' : 'wrong-flash');
            }

            updateProgressAndButtons();
            bindCardEvents(index);
        }

        function updateProgressAndButtons() {
            const idx = state.currentIndex;
            const total = questionsData.length;
            dom.progressBar.style.width = ((idx+1)/total*100) + '%';
            dom.progressText.textContent = `${idx+1}/${total}`;
            dom.btnPrev.disabled = (idx === 0);
            dom.btnNext.disabled = (idx === total-1);

            const submitted = state.submittedFlags[idx];
            if (allSubmitted()) {
                dom.btnSubmit.disabled = true;
                dom.btnSubmit.textContent = '已全部完成';
            } else if (submitted) {
                dom.btnSubmit.disabled = true;
                dom.btnSubmit.textContent = '已提交';
            } else {
                dom.btnSubmit.disabled = false;
                dom.btnSubmit.textContent = '提交本题';
            }
        }

        function bindCardEvents(index) {
            const card = dom.card;
            card.onclick = function(e) {
                if (state.submittedFlags[index]) return;
                const btn = e.target.closest('button');
                if (!btn) return;
                const q = questionsData[index];

                if (q.type === 'single') {
                    const optIndex = btn.getAttribute('data-opt');
                    if (optIndex !== null) {
                        state.userAnswers[index] = parseInt(optIndex, 10);
                        render();
                    }
                } else if (q.type === 'judge') {
                    const val = btn.getAttribute('data-judge');
                    if (val !== null) {
                        state.userAnswers[index] = (val === 'true');
                        render();
                    }
                } else if (q.type === 'multiple') {
                    const optIndex = btn.getAttribute('data-opt');
                    if (optIndex !== null) {
                        const i = parseInt(optIndex, 10);
                        if (!(state.userAnswers[index] instanceof Set)) state.userAnswers[index] = new Set();
                        const s = state.userAnswers[index];
                        s.has(i) ? s.delete(i) : s.add(i);
                        render();
                    }
                }
            };

            if (questionsData[index].type === 'fill') {
                const blankCount = questionsData[index].answers.length;
                for (let bi = 0; bi < blankCount; bi++) {
                    const input = document.getElementById('fillInput' + index + '_' + bi);
                    if (input && !state.submittedFlags[index]) {
                        input.oninput = function() {
                            state.userAnswers[index][bi] = this.value;
                            if (this.value.trim() !== '') {
                                this.classList.add('has-value');
                            } else {
                                this.classList.remove('has-value');
                            }
                        };
                    }
                }
            }
        }

        function submitCurrent() {
            const idx = state.currentIndex;
            if (state.submittedFlags[idx]) return;
            state.submittedFlags[idx] = true;
            render();
            if (allSubmitted()) {
                showScoreIfAllSubmitted();
            }
        }

        function goToPrev() {
            if (state.currentIndex > 0) {
                state.currentIndex--;
                render();
                dom.card.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        }
        function goToNext() {
            if (state.currentIndex < questionsData.length - 1) {
                state.currentIndex++;
                render();
                dom.card.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        }

        function resetAll() {
            state.currentIndex = 0;
            state.userAnswers = questionsData.map(q => {
                if (q.type === 'single') return null;
                if (q.type === 'fill') return new Array(q.answers ? q.answers.length : 1).fill('');
                if (q.type === 'judge') return null;
                if (q.type === 'multiple') return new Set();
                return null;
            });
            state.submittedFlags = new Array(questionsData.length).fill(false);
            hideScore();
            dom.card.classList.remove('correct-flash', 'wrong-flash');
            render();
            window.scrollTo({ top: 0, behavior: 'smooth' });
        }

        dom.btnPrev.addEventListener('click', goToPrev);
        dom.btnNext.addEventListener('click', goToNext);
        dom.btnSubmit.addEventListener('click', submitCurrent);
        dom.btnReset.addEventListener('click', resetAll);

        document.addEventListener('keydown', function(e) {
            const activeEl = document.activeElement;
            if (activeEl && (activeEl.tagName === 'INPUT' || activeEl.tagName === 'TEXTAREA')) return;
            if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') { e.preventDefault(); goToPrev(); }
            else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') { e.preventDefault(); goToNext(); }
            else if (e.key === 'Enter') { e.preventDefault(); submitCurrent(); }
        });

        render();
    })();
</script>
</body>
</html>"###;

        let mut html = String::from(tmpl);
        let theme = &self.theme;
        let replacements: Vec<(&str, &str)> = vec![
            ("{{bg}}", &theme.bg),
            ("{{card_bg}}", &theme.card_bg),
            ("{{primary_light}}", &theme.primary_light),
            ("{{primary}}", &theme.primary),
            ("{{primary_mid}}", &theme.primary_mid),
            ("{{primary_dark}}", &theme.primary_dark),
            ("{{primary_deep}}", &theme.primary_deep),
            ("{{border}}", &theme.border),
            ("{{shadow}}", &theme.shadow),
            ("{{shadow_hover}}", &theme.shadow_hover),
            ("{{text}}", &theme.text),
            ("{{text_light}}", &theme.text_light),
            ("{{correct_bg}}", &theme.correct_bg),
            ("{{correct_border}}", &theme.correct_border),
            ("{{wrong_bg}}", &theme.wrong_bg),
            ("{{wrong_border}}", &theme.wrong_border),
            ("{{highlight_bg}}", &theme.highlight_bg),
            ("{{highlight_border}}", &theme.highlight_border),
            ("{{highlight_text}}", &theme.highlight_text),
            ("{{btn_prev_bg}}", &theme.btn_prev_bg),
            ("{{btn_prev_hover}}", &theme.btn_prev_hover),
            ("{{btn_submit_bg}}", &theme.btn_submit_bg),
            ("{{btn_submit_hover}}", &theme.btn_submit_hover),
            ("{{btn_next_bg}}", &theme.btn_next_bg),
            ("{{btn_next_hover}}", &theme.btn_next_hover),
            ("{{btn_reset_bg}}", &theme.btn_reset_bg),
            ("{{btn_reset_hover}}", &theme.btn_reset_hover),
            ("{{progress_bg}}", &theme.progress_bg),
            ("{{progress_gradient_start}}", &theme.progress_gradient_start),
            ("{{progress_gradient_end}}", &theme.progress_gradient_end),
            ("{{question_bg}}", &theme.question_bg),
            ("{{question_border}}", &theme.question_border),
            ("{{badge_bg}}", &theme.badge_bg),
            ("{{badge_single_bg}}", &theme.badge_single_bg),
            ("{{badge_single_text}}", &theme.badge_single_text),
            ("{{badge_fill_bg}}", &theme.badge_fill_bg),
            ("{{badge_fill_text}}", &theme.badge_fill_text),
            ("{{badge_judge_bg}}", &theme.badge_judge_bg),
            ("{{badge_judge_text}}", &theme.badge_judge_text),
            ("{{badge_multiple_bg}}", &theme.badge_multiple_bg),
            ("{{badge_multiple_text}}", &theme.badge_multiple_text),
            ("{{option_border}}", &theme.option_border),
            ("{{option_hover_border}}", &theme.option_hover_border),
            ("{{option_hover_bg}}", &theme.option_hover_bg),
            ("{{option_letter_bg}}", &theme.option_letter_bg),
            ("{{option_selected_border}}", &theme.option_selected_border),
            ("{{option_selected_bg}}", &theme.option_selected_bg),
            ("{{option_correct_border}}", &theme.option_correct_border),
            ("{{option_correct_bg}}", &theme.option_correct_bg),
            ("{{option_wrong_border}}", &theme.option_wrong_border),
            ("{{option_wrong_bg}}", &theme.option_wrong_bg),
            ("{{option_missed_border}}", &theme.option_missed_border),
            ("{{option_missed_bg}}", &theme.option_missed_bg),
            ("{{fill_input_border}}", &theme.fill_input_border),
            ("{{fill_input_focus_border}}", &theme.fill_input_focus_border),
            ("{{fill_input_focus_shadow}}", &theme.fill_input_focus_shadow),
            ("{{fill_input_has_value_border}}", &theme.fill_input_has_value_border),
            ("{{fill_input_has_value_shadow}}", &theme.fill_input_has_value_shadow),
            ("{{fill_hint_text}}", &theme.fill_hint_text),
            ("{{fill_correct_text}}", &theme.fill_correct_text),
            ("{{judge_border}}", &theme.judge_border),
            ("{{judge_hover_border}}", &theme.judge_hover_border),
            ("{{judge_selected_border}}", &theme.judge_selected_border),
            ("{{judge_selected_bg}}", &theme.judge_selected_bg),
            ("{{judge_correct_border}}", &theme.judge_correct_border),
            ("{{judge_correct_bg}}", &theme.judge_correct_bg),
            ("{{judge_wrong_border}}", &theme.judge_wrong_border),
            ("{{judge_wrong_bg}}", &theme.judge_wrong_bg),
            ("{{score_bg}}", &theme.score_bg),
            ("{{score_border}}", &theme.score_border),
            ("{{score_num_color}}", &theme.score_num_color),
            ("{{score_total_color}}", &theme.score_total_color),
            ("{{time_color}}", &theme.time_color),
        ];
        for (key, value) in &replacements {
            html = html.replace(key, value);
        }
        html.replace("{{PAGE_TITLE}}", title)
            .replace("{{QUESTIONS_DATA}}", questions_json)
            .replace("{{GENERATE_TIME}}", generate_time)
    }
}

pub fn all_templates() -> Vec<Arc<dyn Template>> {
    // 从嵌入的 JSON 文件加载预设主题
    let themes = [
        include_str!("../../assets/themes/default.json"),
        include_str!("../../assets/themes/orange.json"),
        include_str!("../../assets/themes/green.json"),
        include_str!("../../assets/themes/red.json"),
        include_str!("../../assets/themes/pink.json"),
        include_str!("../../assets/themes/yellow.json"),
        include_str!("../../assets/themes/blue.json"),
        include_str!("../../assets/themes/purple.json"),
        include_str!("../../assets/themes/gray.json"),
    ];

    themes
        .iter()
        .filter_map(|json| Theme::from_json(json).ok())
        .map(|theme| Arc::new(QuizTemplate::new(theme)) as Arc<dyn Template>)
        .collect()
}