import { AppTab } from '../types';

interface SidebarProps {
  activeTab: AppTab;
  onTabChange: (tab: AppTab) => void;
  t: (key: string) => string;
}

export default function Sidebar({ activeTab, onTabChange, t }: SidebarProps) {
  const tabs: { id: AppTab; label: string }[] = [
    { id: 'main', label: t('sidebar.import') },
    { id: 'help', label: t('sidebar.help') },
    { id: 'settings', label: t('sidebar.settings') },
  ];

  return (
    <div className="sidebar">
      <div className="sidebar-header">
        QuizForge
      </div>
      <nav className="sidebar-nav">
        {tabs.map(tab => (
          <button
            key={tab.id}
            className={`sidebar-item ${activeTab === tab.id ? 'active' : ''}`}
            onClick={() => onTabChange(tab.id)}
          >
            {tab.label}
          </button>
        ))}
      </nav>
      <div className="sidebar-footer">{t('sidebar.version')}</div>
    </div>
  );
}
