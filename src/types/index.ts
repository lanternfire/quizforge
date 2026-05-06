export interface Question {
  type: 'single' | 'multiple' | 'fill' | 'judge';
  question: string;
  options?: string[];
  correct?: number;
  correct_list?: number[];
  answers?: string[][];
  correct_bool?: boolean;
}

export interface QuizTask {
  id: string;
  inputPath: string;
  outputPath: string;
  pageTitle: string;
  selectedThemeIdx: number | null;
  displayTime: boolean;
  useCurrentTime: boolean;
  customTime: string;
  filenameAddTime: boolean;
  status: string;
  questions?: Question[];
}

export interface Theme {
  name: string;
  bg: string;
  card_bg: string;
  primary_light: string;
  primary: string;
  primary_mid: string;
  primary_dark: string;
  primary_deep: string;
  border: string;
  shadow: string;
  shadow_hover: string;
  text: string;
  text_light: string;
  correct_bg: string;
  correct_border: string;
  wrong_bg: string;
  wrong_border: string;
  highlight_bg: string;
  highlight_border: string;
  highlight_text: string;
  btn_prev_bg: string;
  btn_prev_hover: string;
  btn_submit_bg: string;
  btn_submit_hover: string;
  btn_next_bg: string;
  btn_next_hover: string;
  btn_reset_bg: string;
  btn_reset_hover: string;
  progress_bg: string;
  progress_gradient_start: string;
  progress_gradient_end: string;
  question_bg: string;
  question_border: string;
  badge_bg: string;
  badge_single_bg: string;
  badge_single_text: string;
  badge_fill_bg: string;
  badge_fill_text: string;
  badge_judge_bg: string;
  badge_judge_text: string;
  badge_multiple_bg: string;
  badge_multiple_text: string;
  option_border: string;
  option_hover_border: string;
  option_hover_bg: string;
  option_letter_bg: string;
  option_selected_border: string;
  option_selected_bg: string;
  option_correct_border: string;
  option_correct_bg: string;
  option_wrong_border: string;
  option_wrong_bg: string;
  option_missed_border: string;
  option_missed_bg: string;
  fill_input_border: string;
  fill_input_focus_border: string;
  fill_input_focus_shadow: string;
  fill_input_has_value_border: string;
  fill_input_has_value_shadow: string;
  fill_hint_text: string;
  fill_correct_text: string;
  judge_border: string;
  judge_hover_border: string;
  judge_selected_border: string;
  judge_selected_bg: string;
  judge_correct_border: string;
  judge_correct_bg: string;
  judge_wrong_border: string;
  judge_wrong_bg: string;
  score_bg: string;
  score_border: string;
  score_num_color: string;
  score_total_color: string;
  time_color: string;
}

export interface LocaleDict {
  [key: string]: string;
}

export type AppTab = 'main' | 'help' | 'settings';
export type Lang = 'zh_cn' | 'en_us';
export type ThemeMode = 'light' | 'dark';
