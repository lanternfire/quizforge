import { QuizTask, Theme } from '../types';

interface TaskCardProps {
  task: QuizTask;
  themes: Theme[];
  globalThemeIdx: number;
  t: (key: string) => string;
  onUpdate: (id: string, updates: Partial<QuizTask>) => void;
  onDelete: (id: string) => void;
  onSelectOutputDir: () => void;
  onPreview: (task: QuizTask) => void;
}

export default function TaskCard({
  task, themes, globalThemeIdx, t,
  onUpdate, onDelete, onSelectOutputDir, onPreview
}: TaskCardProps) {
  const statusColor = task.status.startsWith('[OK]') ? '#16a34a' :
    task.status.startsWith('[FAIL]') ? '#dc2626' : '#64748b';

  return (
    <div className="task-card" onClick={() => onPreview(task)} style={{ cursor: 'pointer' }}>
      <div className="task-card-header">
        <span className="task-title">{task.pageTitle || task.inputPath.split(/[\\/]/).pop()}</span>
        <span className="task-status" style={{ color: statusColor }}>
          {task.status || t('task.ready')}
        </span>
        <button className="btn-icon" onClick={(e) => { e.stopPropagation(); onDelete(task.id); }} title={t('task.delete')}>
          X
        </button>
      </div>
      <div className="task-card-body" onClick={e => e.stopPropagation()}>
        {/* Source file */}
        <div className="task-row">
          <label>{t('task.source')}</label>
          <span className="task-value mono">{task.inputPath.split(/[\\/]/).pop()}</span>
        </div>
        
        {/* Page title (editable) */}
        <div className="task-row">
          <label>{t('task.title')}</label>
          <input
            type="text"
            className="text-input"
            value={task.pageTitle}
            onChange={(e) => onUpdate(task.id, { pageTitle: e.target.value })}
          />
        </div>

        {/* Output path */}
        <div className="task-row">
          <label>{t('task.output')}</label>
          <div className="output-path-group">
            <input
              type="text"
              className="text-input"
              value={task.outputPath}
              onChange={(e) => onUpdate(task.id, { outputPath: e.target.value })}
            />
            <button className="btn btn-small" onClick={onSelectOutputDir}>...</button>
          </div>
        </div>

        {/* Theme per task */}
        <div className="task-row">
          <label>{t('task.theme')}</label>
          <select
            className="select-input"
            value={task.selectedThemeIdx ?? -1}
            onChange={(e) => {
              const val = Number(e.target.value);
              const selectedIdx = val === -1 ? null : val;
              onUpdate(task.id, { selectedThemeIdx: selectedIdx });
              // Trigger preview refresh after theme change
              requestAnimationFrame(() => {
                onPreview({ ...task, selectedThemeIdx: selectedIdx });
              });
            }}
          >
            <option value={-1}>{t('task.follow_global')} ({themes[globalThemeIdx]?.name})</option>
            {themes.map((th, i) => (
              <option key={i} value={i}>{th.name}</option>
            ))}
          </select>
        </div>

        {/* Display time toggle */}
        <div className="task-row">
          <label>{t('task.display_time')}</label>
          <div className="toggle-group">
            <button
              className={`toggle-btn-sm ${task.displayTime ? 'active' : ''}`}
              onClick={() => onUpdate(task.id, { displayTime: !task.displayTime })}
            >
              {task.displayTime ? t('common.yes') : t('common.no')}
            </button>
          </div>
        </div>

        {/* Time options */}
        {task.displayTime && (
          <>
            <div className="task-row">
              <label>{t('task.time_source')}</label>
              <div className="toggle-group">
                <button
                  className={`toggle-btn-sm ${task.useCurrentTime ? 'active' : ''}`}
                  onClick={() => onUpdate(task.id, { useCurrentTime: true })}
                >
                  {t('task.current_time')}
                </button>
                <button
                  className={`toggle-btn-sm ${!task.useCurrentTime ? 'active' : ''}`}
                  onClick={() => onUpdate(task.id, { useCurrentTime: false })}
                >
                  {t('task.custom_time')}
                </button>
              </div>
            </div>
            {!task.useCurrentTime && (
              <div className="task-row">
                <label>{t('task.custom_time')}</label>
                <input
                  type="text"
                  className="text-input"
                  value={task.customTime}
                  onChange={(e) => onUpdate(task.id, { customTime: e.target.value })}
                  placeholder="2025-01-01 12:00:00"
                />
              </div>
            )}
          </>
        )}

        {/* Filename add time toggle */}
        <div className="task-row">
          <label>{t('task.filename_add_time')}</label>
          <div className="toggle-group">
            <button
              className={`toggle-btn-sm ${task.filenameAddTime ? 'active' : ''}`}
              onClick={() => onUpdate(task.id, { filenameAddTime: !task.filenameAddTime })}
            >
              {task.filenameAddTime ? t('common.yes') : t('common.no')}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
