import { useTranslation } from '../i18n';
import { BarChart3, Zap, Gamepad2 } from 'lucide-react';

interface SidebarProps {
  activeTab: string;
  onSelectTab: (tab: string) => void;
  dualsenseExtensionEnabled?: boolean;
  dualsenseConnected?: boolean;
}

export function Sidebar({
  activeTab,
  onSelectTab,
  dualsenseExtensionEnabled,
  dualsenseConnected,
}: SidebarProps) {
  const { t } = useTranslation();

  const tabs = [
    { id: 'overview', label: t.tab_overview, icon: BarChart3 },
    { id: 'steam-bridge', label: t.tab_steam, icon: Zap },
    {
      id: 'dualsense',
      label: t.tab_dualsense,
      icon: Gamepad2,
      badge: !dualsenseExtensionEnabled ? 'OFF' : dualsenseConnected ? 'READY' : 'ON',
      badgeActive: Boolean(dualsenseExtensionEnabled && dualsenseConnected),
    },
  ];

  return (
    <nav className="deck-sidebar">
      <ul className="nav-list" role="tablist">
        {tabs.map((tab, idx) => {
          const Icon = tab.icon;
          const isActive = activeTab === tab.id;
          return (
            <li
              key={tab.id}
              className={`nav-item focusable ${isActive ? 'active' : ''}`}
              onClick={() => onSelectTab(tab.id)}
              tabIndex={idx + 2}
              role="tab"
              aria-selected={isActive}
            >
              <div className="nav-item-left">
                <Icon size={16} />
                <span>{tab.label}</span>
              </div>
              {tab.badge && (
                <span className={`nav-badge ${tab.badgeActive ? 'active' : ''}`}>
                  {tab.badge}
                </span>
              )}
            </li>
          );
        })}
      </ul>

      <div className="sidebar-footer">
        <div className="service-pill">
          <span className="badge-dot" />
          <span>v0.1.0 • STEAM DECK & HTPC</span>
        </div>
      </div>
    </nav>
  );
}

