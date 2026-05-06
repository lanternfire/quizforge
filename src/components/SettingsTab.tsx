import { Theme, Lang } from '../types';

interface SettingsTabProps {
  themes: Theme[];
  globalThemeIdx: number;
  lang: Lang;
  themeMode: 'light' | 'dark';
  t: (key: string) => string;
  onChangeLang: (l: Lang) => void;
  onChangeGlobalTheme: (idx: number) => void;
  onChangeThemeMode: (mode: 'light' | 'dark') => void;
  onApplyAll: () => void;
}

export default function SettingsTab({
  themes, globalThemeIdx, lang, themeMode, t,
  onChangeLang, onChangeGlobalTheme, onChangeThemeMode, onApplyAll
}: SettingsTabProps) {
  return (
    <div className="tab-content">
      <div className="tab-content-inner">
        <div className="settings-tab">
          <h2>{t('settings.title')}</h2>
          <div className="settings-section">
            <label>{t('settings.global_theme')}</label>
            <select
              value={globalThemeIdx}
              onChange={(e) => onChangeGlobalTheme(Number(e.target.value))}
              className="select-input"
            >
              {themes.map((th, i) => (
                <option key={i} value={i}>{th.name}</option>
              ))}
            </select>
            <button className="btn btn-secondary" onClick={onApplyAll}>
              {t('settings.apply_all')}
            </button>
          </div>
          <div className="settings-section">
            <label>{t('settings.language')}</label>
            <div className="toggle-group">
              <button
                className={`toggle-btn ${lang === 'zh_cn' ? 'active' : ''}`}
                onClick={() => onChangeLang('zh_cn')}
              >
                中文
              </button>
              <button
                className={`toggle-btn ${lang === 'en_us' ? 'active' : ''}`}
                onClick={() => onChangeLang('en_us')}
              >
                English
              </button>
            </div>
          </div>
          <div className="settings-section">
            <label>{t('settings.theme_mode')}</label>
            <div className="toggle-group">
              <button
                className={`toggle-btn ${themeMode === 'light' ? 'active' : ''}`}
                onClick={() => onChangeThemeMode('light')}
              >
                {t('settings.light')}
              </button>
              <button
                className={`toggle-btn ${themeMode === 'dark' ? 'active' : ''}`}
                onClick={() => onChangeThemeMode('dark')}
              >
                {t('settings.dark')}
              </button>
            </div>
          </div>
          <div className="settings-section">
            <label>{t('settings.import_theme')}</label>
            <button className="btn btn-secondary">{t('settings.import_theme')}</button>
          </div>
          <div className="settings-section">
            <label>{t('settings.export_theme')}</label>
            <button className="btn btn-secondary">{t('settings.export_theme')}</button>
          </div>
          <div className="version-info">
            QuizForge v2.0.0
          </div>
        </div>
      </div>
    </div>
  );
}
