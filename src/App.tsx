import { useState, useCallback, useEffect, useRef } from 'react';
import { QuizTask, Theme, AppTab, Lang } from './types';
import { useLocale } from './hooks/useLocale';
import Sidebar from './components/Sidebar';
import TaskCard from './components/TaskCard';
import Preview from './components/Preview';
import HelpTab from './components/HelpTab';
import SettingsTab from './components/SettingsTab';
import './styles/global.css';

// Default themes for dev/fallback
function getDefaultThemes(): Theme[] {
  return [
    { name: '默认(蓝)', bg: '#f0f7fc', card_bg: '#ffffff', primary_light: '#e8f0fe', primary: '#2b7be4', primary_mid: '#4a90d9', primary_dark: '#1a5bbf', primary_deep: '#123e7a', border: '#e2e8f0', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#e0f2fe', highlight_border: '#7dd3fc', highlight_text: '#0369a1', btn_prev_bg: '#f1f5f9', btn_prev_hover: '#e2e8f0', btn_submit_bg: '#2563eb', btn_submit_hover: '#1d4ed8', btn_next_bg: '#f1f5f9', btn_next_hover: '#e2e8f0', btn_reset_bg: '#f8fafc', btn_reset_hover: '#f1f5f9', progress_bg: '#e2e8f0', progress_gradient_start: '#3b82f6', progress_gradient_end: '#60a5fa', question_bg: '#fafbfc', question_border: '#e8eaed', badge_bg: '#e0e7ff', badge_single_bg: '#dbeafe', badge_single_text: '#1e40af', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#93c5fd', option_hover_bg: '#f0f7ff', option_letter_bg: '#eef2f6', option_selected_border: '#3b82f6', option_selected_bg: '#eff6ff', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#60a5fa', fill_input_focus_shadow: 'rgba(96,165,250,0.15)', fill_input_has_value_border: '#60a5fa', fill_input_has_value_shadow: 'rgba(96,165,250,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#93c5fd', judge_selected_border: '#3b82f6', judge_selected_bg: '#eff6ff', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#f0f9ff', score_border: '#bae6fd', score_num_color: '#0284c7', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '橙色', bg: '#fdf4ed', card_bg: '#ffffff', primary_light: '#ffedd5', primary: '#f97316', primary_mid: '#fb923c', primary_dark: '#ea580c', primary_deep: '#9a3412', border: '#fde68a', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#ffedd5', highlight_border: '#fdba74', highlight_text: '#c2410c', btn_prev_bg: '#fff7ed', btn_prev_hover: '#ffedd5', btn_submit_bg: '#f97316', btn_submit_hover: '#ea580c', btn_next_bg: '#fff7ed', btn_next_hover: '#ffedd5', btn_reset_bg: '#ffffff', btn_reset_hover: '#fef3c7', progress_bg: '#fde68a', progress_gradient_start: '#f97316', progress_gradient_end: '#fb923c', question_bg: '#fafbfc', question_border: '#fde68a', badge_bg: '#ffedd5', badge_single_bg: '#fef3c7', badge_single_text: '#92400e', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#fdba74', option_hover_bg: '#fff7ed', option_letter_bg: '#eef2f6', option_selected_border: '#f97316', option_selected_bg: '#fff7ed', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#fb923c', fill_input_focus_shadow: 'rgba(251,146,60,0.15)', fill_input_has_value_border: '#fb923c', fill_input_has_value_shadow: 'rgba(251,146,60,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#fdba74', judge_selected_border: '#f97316', judge_selected_bg: '#fff7ed', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#fff7ed', score_border: '#fdba74', score_num_color: '#c2410c', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '绿色', bg: '#f0fdf4', card_bg: '#ffffff', primary_light: '#dcfce7', primary: '#16a34a', primary_mid: '#22c55e', primary_dark: '#15803d', primary_deep: '#14532d', border: '#bbf7d0', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#dcfce7', highlight_border: '#86efac', highlight_text: '#166534', btn_prev_bg: '#f0fdf4', btn_prev_hover: '#dcfce7', btn_submit_bg: '#16a34a', btn_submit_hover: '#15803d', btn_next_bg: '#f0fdf4', btn_next_hover: '#dcfce7', btn_reset_bg: '#ffffff', btn_reset_hover: '#fef2f2', progress_bg: '#bbf7d0', progress_gradient_start: '#16a34a', progress_gradient_end: '#4ade80', question_bg: '#fafbfc', question_border: '#bbf7d0', badge_bg: '#dcfce7', badge_single_bg: '#dcfce7', badge_single_text: '#166534', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#86efac', option_hover_bg: '#f0fdf4', option_letter_bg: '#eef2f6', option_selected_border: '#16a34a', option_selected_bg: '#f0fdf4', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#4ade80', fill_input_focus_shadow: 'rgba(74,222,128,0.15)', fill_input_has_value_border: '#4ade80', fill_input_has_value_shadow: 'rgba(74,222,128,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#86efac', judge_selected_border: '#16a34a', judge_selected_bg: '#f0fdf4', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#f0fdf4', score_border: '#86efac', score_num_color: '#15803d', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '红色', bg: '#fef2f2', card_bg: '#ffffff', primary_light: '#fce7f3', primary: '#dc2626', primary_mid: '#ef4444', primary_dark: '#b91c1c', primary_deep: '#7f1d1d', border: '#fecaca', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#fce7f3', highlight_border: '#f9a8d4', highlight_text: '#be185d', btn_prev_bg: '#fef2f2', btn_prev_hover: '#fce7f3', btn_submit_bg: '#dc2626', btn_submit_hover: '#b91c1c', btn_next_bg: '#fef2f2', btn_next_hover: '#fce7f3', btn_reset_bg: '#ffffff', btn_reset_hover: '#fef2f2', progress_bg: '#fecaca', progress_gradient_start: '#dc2626', progress_gradient_end: '#ef4444', question_bg: '#fafbfc', question_border: '#fecaca', badge_bg: '#fce7f3', badge_single_bg: '#fce7f3', badge_single_text: '#be185d', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#f9a8d4', option_hover_bg: '#fdf2f8', option_letter_bg: '#eef2f6', option_selected_border: '#dc2626', option_selected_bg: '#fef2f2', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#f87171', fill_input_focus_shadow: 'rgba(248,113,113,0.15)', fill_input_has_value_border: '#f87171', fill_input_has_value_shadow: 'rgba(248,113,113,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#f9a8d4', judge_selected_border: '#dc2626', judge_selected_bg: '#fef2f2', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#fef2f2', score_border: '#fecaca', score_num_color: '#b91c1c', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '粉色', bg: '#fdf2f8', card_bg: '#ffffff', primary_light: '#fce7f3', primary: '#ec4899', primary_mid: '#f472b6', primary_dark: '#db2777', primary_deep: '#9d174d', border: '#fbcfe8', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#fce7f3', highlight_border: '#f9a8d4', highlight_text: '#be185d', btn_prev_bg: '#fdf2f8', btn_prev_hover: '#fce7f3', btn_submit_bg: '#ec4899', btn_submit_hover: '#db2777', btn_next_bg: '#fdf2f8', btn_next_hover: '#fce7f3', btn_reset_bg: '#ffffff', btn_reset_hover: '#fdf2f8', progress_bg: '#fbcfe8', progress_gradient_start: '#ec4899', progress_gradient_end: '#f472b6', question_bg: '#fafbfc', question_border: '#fbcfe8', badge_bg: '#fce7f3', badge_single_bg: '#fce7f3', badge_single_text: '#be185d', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#f9a8d4', option_hover_bg: '#fdf2f8', option_letter_bg: '#eef2f6', option_selected_border: '#ec4899', option_selected_bg: '#fdf2f8', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#f472b6', fill_input_focus_shadow: 'rgba(244,114,182,0.15)', fill_input_has_value_border: '#f472b6', fill_input_has_value_shadow: 'rgba(244,114,182,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#f9a8d4', judge_selected_border: '#ec4899', judge_selected_bg: '#fdf2f8', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#fdf2f8', score_border: '#fbcfe8', score_num_color: '#db2777', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '黄色', bg: '#fefce8', card_bg: '#ffffff', primary_light: '#fef9c3', primary: '#ca8a04', primary_mid: '#eab308', primary_dark: '#a16207', primary_deep: '#713f12', border: '#fde68a', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#fef9c3', highlight_border: '#fde047', highlight_text: '#854d0e', btn_prev_bg: '#fefce8', btn_prev_hover: '#fef9c3', btn_submit_bg: '#ca8a04', btn_submit_hover: '#a16207', btn_next_bg: '#fefce8', btn_next_hover: '#fef9c3', btn_reset_bg: '#ffffff', btn_reset_hover: '#fefce8', progress_bg: '#fde68a', progress_gradient_start: '#ca8a04', progress_gradient_end: '#eab308', question_bg: '#fafbfc', question_border: '#fde68a', badge_bg: '#fef9c3', badge_single_bg: '#fef9c3', badge_single_text: '#854d0e', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#fde047', option_hover_bg: '#fefce8', option_letter_bg: '#eef2f6', option_selected_border: '#ca8a04', option_selected_bg: '#fefce8', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#eab308', fill_input_focus_shadow: 'rgba(234,179,8,0.15)', fill_input_has_value_border: '#eab308', fill_input_has_value_shadow: 'rgba(234,179,8,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#fde047', judge_selected_border: '#ca8a04', judge_selected_bg: '#fefce8', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#fefce8', score_border: '#fde68a', score_num_color: '#a16207', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '紫色', bg: '#faf5ff', card_bg: '#ffffff', primary_light: '#f3e8ff', primary: '#9333ea', primary_mid: '#a855f7', primary_dark: '#7e22ce', primary_deep: '#581c87', border: '#e9d5ff', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#f3e8ff', highlight_border: '#d8b4fe', highlight_text: '#7e22ce', btn_prev_bg: '#faf5ff', btn_prev_hover: '#f3e8ff', btn_submit_bg: '#9333ea', btn_submit_hover: '#7e22ce', btn_next_bg: '#faf5ff', btn_next_hover: '#f3e8ff', btn_reset_bg: '#ffffff', btn_reset_hover: '#faf5ff', progress_bg: '#e9d5ff', progress_gradient_start: '#9333ea', progress_gradient_end: '#a855f7', question_bg: '#fafbfc', question_border: '#e9d5ff', badge_bg: '#f3e8ff', badge_single_bg: '#f3e8ff', badge_single_text: '#7e22ce', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#d8b4fe', option_hover_bg: '#faf5ff', option_letter_bg: '#eef2f6', option_selected_border: '#9333ea', option_selected_bg: '#faf5ff', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#a855f7', fill_input_focus_shadow: 'rgba(168,85,247,0.15)', fill_input_has_value_border: '#a855f7', fill_input_has_value_shadow: 'rgba(168,85,247,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#d8b4fe', judge_selected_border: '#9333ea', judge_selected_bg: '#faf5ff', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#faf5ff', score_border: '#e9d5ff', score_num_color: '#7e22ce', score_total_color: '#64748b', time_color: '#94a3b8' },
    { name: '灰色', bg: '#f8fafc', card_bg: '#ffffff', primary_light: '#f1f5f9', primary: '#64748b', primary_mid: '#94a3b8', primary_dark: '#475569', primary_deep: '#1e293b', border: '#e2e8f0', shadow: '0 2px 12px rgba(0,0,0,0.06)', shadow_hover: '0 6px 24px rgba(0,0,0,0.1)', text: '#1f2937', text_light: '#64748b', correct_bg: '#f0fdf4', correct_border: '#86efac', wrong_bg: '#fef2f2', wrong_border: '#fca5a5', highlight_bg: '#f1f5f9', highlight_border: '#cbd5e1', highlight_text: '#475569', btn_prev_bg: '#f8fafc', btn_prev_hover: '#f1f5f9', btn_submit_bg: '#64748b', btn_submit_hover: '#475569', btn_next_bg: '#f8fafc', btn_next_hover: '#f1f5f9', btn_reset_bg: '#ffffff', btn_reset_hover: '#f8fafc', progress_bg: '#e2e8f0', progress_gradient_start: '#64748b', progress_gradient_end: '#94a3b8', question_bg: '#fafbfc', question_border: '#e2e8f0', badge_bg: '#f1f5f9', badge_single_bg: '#f1f5f9', badge_single_text: '#475569', badge_fill_bg: '#dcfce7', badge_fill_text: '#166534', badge_judge_bg: '#f3e8ff', badge_judge_text: '#6b21a8', badge_multiple_bg: '#fef9c3', badge_multiple_text: '#854d0e', option_border: '#d1d5db', option_hover_border: '#cbd5e1', option_hover_bg: '#f8fafc', option_letter_bg: '#eef2f6', option_selected_border: '#64748b', option_selected_bg: '#f8fafc', option_correct_border: '#22c55e', option_correct_bg: '#f0fdf4', option_wrong_border: '#ef4444', option_wrong_bg: '#fef2f2', option_missed_border: '#eab308', option_missed_bg: '#fefce8', fill_input_border: '#d1d5db', fill_input_focus_border: '#94a3b8', fill_input_focus_shadow: 'rgba(148,163,184,0.15)', fill_input_has_value_border: '#94a3b8', fill_input_has_value_shadow: 'rgba(148,163,184,0.1)', fill_hint_text: '#9ca3af', fill_correct_text: '#16a34a', judge_border: '#d1d5db', judge_hover_border: '#cbd5e1', judge_selected_border: '#64748b', judge_selected_bg: '#f8fafc', judge_correct_border: '#22c55e', judge_correct_bg: '#f0fdf4', judge_wrong_border: '#ef4444', judge_wrong_bg: '#fef2f2', score_bg: '#f8fafc', score_border: '#e2e8f0', score_num_color: '#475569', score_total_color: '#64748b', time_color: '#94a3b8' },
  ];
}

function generateId(): string {
  return Math.random().toString(36).substr(2, 9);
}

async function readFileViaBrowser(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = reject;
    reader.readAsText(file);
  });
}

function extractTitle(content: string): string {
  const lines = content.split('\n');
  for (const line of lines) {
    if (line.startsWith('标题：') || line.startsWith('Title:')) {
      return line.split(/[:：]/)[1]?.trim() || '';
    }
  }
  return '';
}

// Track if we're in Tauri mode
let isTauriMode = false;
try {
  isTauriMode = !!(window as any).__TAURI_INTERNALS__;
} catch {}

export default function App() {
  const { t, lang, changeLang, dict } = useLocale('zh_cn');
  const [activeTab, setActiveTab] = useState<AppTab>('main');
  const [tasks, setTasks] = useState<QuizTask[]>([]);
  const [outputDir, setOutputDir] = useState<string>('');
  const [themes, setThemes] = useState<Theme[]>(getDefaultThemes());
  const [globalThemeIdx, setGlobalThemeIdx] = useState(0);
  const [previewHtml, setPreviewHtml] = useState('');
  const [previewThemeName, setPreviewThemeName] = useState('');
  const [themeMode, setThemeMode] = useState<'light' | 'dark'>('light');
  const isTauri = useRef(isTauriMode);
  const lastDropTime = useRef(0);
  // Store handlePreviewTask in a ref so setTimeout always has latest version
  const previewFnRef = useRef<((task: QuizTask) => Promise<void>) | null>(null);

  // Initialize: try to load themes from Tauri backend
  useEffect(() => {
    async function loadThemes() {
      try {
        if (isTauri.current) {
          const { invoke } = await import('@tauri-apps/api/core');
          const backendThemes = await invoke<Theme[]>('list_themes');
          if (backendThemes && backendThemes.length > 0) {
            setThemes(backendThemes);
          }
        }
      } catch (err) {
        console.log('Using fallback themes');
      }
    }
    loadThemes();
  }, []);

  // Set up Tauri drag-drop listener (v2) — debounced to prevent duplicates
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    async function setupDragDrop() {
      try {
        if (isTauri.current) {
          const { listen } = await import('@tauri-apps/api/event');
          unlisten = await listen<{ paths: string[]; position: { x: number; y: number } }>('tauri://drag-drop', async (event) => {
            // Debounce: skip if duplicate within 500ms
            const now = Date.now();
            if (now - lastDropTime.current < 500) return;
            lastDropTime.current = now;

            const payloadPaths = event.payload as any;
            let paths: string[] = [];
            if (Array.isArray(payloadPaths?.paths)) paths = payloadPaths.paths;
            else if (typeof payloadPaths === 'string') paths = [payloadPaths];

            if (paths.length === 0) return;

            const { invoke } = await import('@tauri-apps/api/core');
            const newTasks: QuizTask[] = [];

            for (const fp of paths) {
              if (!fp.toLowerCase().endsWith('.txt')) continue;
              try {
                const content = await invoke<string>('read_file_content', { path: fp });
                const name = fp.split(/[\\/]/).pop() || '';
                const title = extractTitle(content);
                newTasks.push({
                  id: generateId(),
                  inputPath: fp,
                  outputPath: fp.replace(/\.txt$/i, '') + '.html',
                  pageTitle: title || name.replace(/\.txt$/i, ''),
                  selectedThemeIdx: null,
                  displayTime: false,
                  useCurrentTime: true,
                  customTime: '',
                  filenameAddTime: false,
                  status: '已导入',
                  questions: [],
                } as QuizTask);
              } catch (err) {
                console.error('Error reading dropped file', fp, err);
              }
            }

            if (newTasks.length > 0) {
              setTasks(prev => {
                const updated = [...prev, ...newTasks];
                // Auto-preview first new task on next tick
                setTimeout(() => {
                  if (previewFnRef.current) previewFnRef.current(newTasks[0]);
                }, 100);
                return updated;
              });
              setActiveTab('main');
            }
          });
        }
      } catch (err) {
        console.log('Drag-drop Tauri listener unavailable', err);
      }
    }

    setupDragDrop();
    return () => { if (unlisten) unlisten(); };
  }, []);

  const handleImportFiles = useCallback(async () => {
    try {
      // Try Tauri dialog
      try {
        if (isTauri.current) {
          const { open } = await import('@tauri-apps/plugin-dialog');
          const selected = await open({
            multiple: true,
            filters: [{ name: '题库文件', extensions: ['txt'] }]
          });
          if (!selected) return;
          const files = Array.isArray(selected) ? selected : [selected];
          
          const { invoke } = await import('@tauri-apps/api/core');
          const newTasks: QuizTask[] = [];
          for (const filePath of files) {
            try {
              const content = await invoke<string>('read_file_content', { path: filePath });
              const name = filePath.split(/[\\/]/).pop() || '';
              const title = extractTitle(content);
              newTasks.push({
                id: generateId(),
                inputPath: filePath,
                outputPath: filePath.replace(/\.txt$/i, '') + '.html',
                pageTitle: title || name.replace(/\.txt$/i, ''),
                selectedThemeIdx: null,
                displayTime: false,
                useCurrentTime: true,
                customTime: '',
                filenameAddTime: false,
                status: '已导入',
                questions: [],
              } as QuizTask);
            } catch (err) {
              console.error('Error reading file', filePath, err);
            }
          }
          if (newTasks.length > 0) {
            setTasks(prev => [...prev, ...newTasks]);
            setTimeout(() => {
              if (previewFnRef.current) previewFnRef.current(newTasks[0]);
            }, 100);
          }
          return;
        }
      } catch {
        // Fallback
      }

      // Fallback: browser file input
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = '.txt';
      input.multiple = true;
      input.onchange = async (e) => {
        const files = (e.target as HTMLInputElement).files;
        if (!files) return;
        const newTasks: QuizTask[] = [];
        for (let i = 0; i < files.length; i++) {
          const file = files[i];
          const content = await readFileViaBrowser(file);
          const title = extractTitle(content);
          newTasks.push({
            id: generateId(),
            inputPath: file.name,
            outputPath: file.name.replace(/\.txt$/i, '') + '.html',
            pageTitle: title || file.name.replace(/\.txt$/i, ''),
            selectedThemeIdx: null,
            displayTime: false,
            useCurrentTime: true,
            customTime: '',
            filenameAddTime: false,
            status: '已导入',
            questions: [],
          } as QuizTask);
        }
        setTasks(prev => {
          const updated = [...prev, ...newTasks];
          setTimeout(() => {
            if (previewFnRef.current) previewFnRef.current(newTasks[0]);
          }, 100);
          return updated;
        });
      };
      input.click();
    } catch (err) {
      console.error('Import failed', err);
    }
  }, []);

  const handleSelectOutputDir = useCallback(async () => {
    try {
      if (isTauri.current) {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({ directory: true });
        if (selected && typeof selected === 'string') {
          setOutputDir(selected);
        }
        return;
      }
    } catch {}
    const dir = prompt('输入输出目录路径（留空使用默认）:');
    if (dir) setOutputDir(dir);
  }, []);

  const handleUpdateTask = useCallback((id: string, updates: Partial<QuizTask>) => {
    setTasks(prev => prev.map(t => t.id === id ? { ...t, ...updates } : t));
  }, []);

  const handleDeleteTask = useCallback((id: string) => {
    setTasks(prev => {
      const updated = prev.filter(t => t.id !== id);
      if (updated.length === 0) {
        setPreviewHtml('');
        setPreviewThemeName('');
      }
      return updated;
    });
  }, []);

  // Preview on task select: parse file and generate preview HTML
  const handlePreviewTask = useCallback(async (task: QuizTask) => {
    const themeIdx = task.selectedThemeIdx !== null ? task.selectedThemeIdx : globalThemeIdx;
    setPreviewThemeName(themes[themeIdx]?.name || '');
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const content = await invoke<string>('read_file_content', { path: task.inputPath });
      const [titleOpt, questions] = await invoke<[string | null, any[]]>('parse_file', { content });
      const localeJson = JSON.stringify(dict);
      const html = await invoke<string>('generate_html', {
        title: titleOpt || task.pageTitle,
        questionsJson: JSON.stringify(questions),
        generateTime: '',
        localeJson,
        themeIdx,
      });
      setPreviewHtml(html);
      setPreviewThemeName(themes[themeIdx]?.name || '');
    } catch (err) {
      setPreviewHtml(`<div style="font-family:sans-serif;padding:20px;text-align:center;color:#666;">${task.pageTitle}</div>`);
    }
  }, [globalThemeIdx, themes, dict]);

  // Keep ref updated
  useEffect(() => {
    previewFnRef.current = handlePreviewTask;
  }, [handlePreviewTask]);

  // Re-generate preview when language changes, if there are tasks
  useEffect(() => {
    if (tasks.length > 0 && previewFnRef.current) {
      previewFnRef.current(tasks[0]);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [lang]);

  const handleGenerate = useCallback(async () => {
    for (const task of tasks) {
      try {
        const themeIdx = task.selectedThemeIdx !== null ? task.selectedThemeIdx : globalThemeIdx;
        const theme = themes[themeIdx];

        let outputPath = task.outputPath;
        if (outputDir) {
          const filename = outputPath.split(/[\\/]/).pop() || 'output.html';
          outputPath = outputDir.replace(/\\/g, '/') + '/' + filename;
        }
        if (task.filenameAddTime) {
          const now = new Date();
          const dateStr = now.getFullYear().toString() +
            String(now.getMonth() + 1).padStart(2, '0') +
            String(now.getDate()).padStart(2, '0') + '_';
          const parts = outputPath.split(/[\\/]/);
          const filename = parts.pop() || 'output.html';
          parts.push(dateStr + filename);
          outputPath = parts.join('/');
        }

        try {
          if (isTauri.current) {
            const { invoke } = await import('@tauri-apps/api/core');
            const content = await invoke<string>('read_file_content', { path: task.inputPath });
            const [titleOpt, questions] = await invoke<[string | null, any[]]>('parse_file', { content });
            const localeJson = JSON.stringify(dict);
            let timeStr = '';
            if (task.displayTime) {
              if (task.useCurrentTime) {
                timeStr = await invoke<string>('get_current_time');
              } else {
                timeStr = task.customTime;
              }
            }
            const pageTitle = titleOpt || task.pageTitle;
            const html = await invoke<string>('generate_html', {
              title: pageTitle,
              questionsJson: JSON.stringify(questions),
              generateTime: timeStr,
              localeJson,
              themeIdx,
            });
            await invoke('write_file', { path: outputPath, content: html });
            handleUpdateTask(task.id, { status: `[OK] 生成成功 -> ${outputPath}`, pageTitle });
            setPreviewHtml(html);
            setPreviewThemeName(themes[themeIdx]?.name || '');
            continue;
          }
        } catch (err) {
          console.error('Backend generation failed, using fallback', err);
        }

        setPreviewHtml(`<div style="font-family:sans-serif;padding:20px;text-align:center;color:#666;">${task.pageTitle}</div>`);
        setPreviewThemeName(themes[themeIdx]?.name || '');
        handleUpdateTask(task.id, { status: `[OK] 生成成功 -> ${outputPath}` });
      } catch (err) {
        handleUpdateTask(task.id, { status: `[FAIL] ${err}` });
      }
    }
  }, [tasks, globalThemeIdx, themes, handleUpdateTask, outputDir, dict]);

  const handleApplyAll = useCallback(() => {
    setTasks(prev => prev.map(t => ({ ...t, selectedThemeIdx: null })));
  }, []);

  // Browser-only drag-drop: only fires outside Tauri
  const handleDrop = useCallback((e: React.DragEvent) => {
    if (isTauri.current) return; // Tauri handles drag-drop natively
    e.preventDefault();
    const files = Array.from(e.dataTransfer.files).filter(f => f.name.endsWith('.txt'));
    if (files.length === 0) return;
    Promise.all(files.map(async f => {
      const content = await readFileViaBrowser(f);
      const title = extractTitle(content);
      return {
        id: generateId(),
        inputPath: f.name,
        outputPath: f.name.replace(/\.txt$/i, '') + '.html',
        pageTitle: title || f.name.replace(/\.txt$/i, ''),
        selectedThemeIdx: null,
        displayTime: false,
        useCurrentTime: true,
        customTime: '',
        filenameAddTime: false,
        status: '已导入',
        questions: [],
      } as QuizTask;
    })).then(newTasks => {
      if (newTasks.length > 0) {
        setTasks(prev => [...prev, ...newTasks]);
        setTimeout(() => {
          if (previewFnRef.current) previewFnRef.current(newTasks[0]);
        }, 100);
      }
    });
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    if (isTauri.current) return;
    e.preventDefault();
  }, []);

  return (
    <div className={`app-container ${themeMode}`}>
      <Sidebar activeTab={activeTab} onTabChange={setActiveTab} t={t} />
      <div className="main-content" onDrop={handleDrop} onDragOver={handleDragOver}>
        {activeTab === 'main' && (
          <>
            <div className="main-toolbar">
              <div className="left">
                <button className="btn btn-primary" onClick={handleImportFiles}>
                  {t('main.empty.import_btn')}
                </button>
                {tasks.length > 0 && (
                  <button
                    className="btn btn-accent"
                    onClick={handleGenerate}
                  >
                    {t('generate.btn')} ({tasks.length})
                  </button>
                )}
              </div>
              <div className="right">
                <button className="btn btn-ghost" onClick={handleSelectOutputDir}>
                  {outputDir || t('settings.output_dir')}
                </button>
              </div>
            </div>
            <div className="task-area">
              {tasks.length === 0 ? (
                <div className="empty-state">
                  <h2>{t('main.empty.title')}</h2>
                  <p>{t('main.empty.desc')}</p>
                  <button className="btn btn-primary" onClick={handleImportFiles}>
                    {t('main.empty.import_btn')}
                  </button>
                </div>
              ) : (
                tasks.map(task => (
                  <TaskCard
                    key={task.id}
                    task={task}
                    themes={themes}
                    globalThemeIdx={globalThemeIdx}
                    t={t}
                    onUpdate={handleUpdateTask}
                    onDelete={handleDeleteTask}
                    onSelectOutputDir={handleSelectOutputDir}
                    onPreview={handlePreviewTask}
                  />
                ))
              )}
            </div>
          </>
        )}
        {activeTab === 'help' && <HelpTab t={t} lang={lang} />}
        {activeTab === 'settings' && (
          <SettingsTab
            themes={themes}
            globalThemeIdx={globalThemeIdx}
            lang={lang}
            themeMode={themeMode}
            t={t}
            onChangeLang={changeLang}
            onChangeGlobalTheme={setGlobalThemeIdx}
            onChangeThemeMode={setThemeMode}
            onApplyAll={handleApplyAll}
          />
        )}
      </div>
      {activeTab === 'main' && previewHtml && (
        <Preview htmlContent={previewHtml} themeName={previewThemeName} t={t} themeMode={themeMode} />
      )}
    </div>
  );
}
