import { useTranslation } from '../../i18n';
import { StatusResponse } from '../../types/models';
import { Zap, Gamepad2, Search, Info } from 'lucide-react';

interface OverviewTabProps {
  status: StatusResponse | null;
  onNavigateTab: (tab: string) => void;
  onSyncSteam: () => void;
  onScanGames: () => void;
}

export function OverviewTab({
  status,
  onNavigateTab,
  onSyncSteam,
  onScanGames,
}: OverviewTabProps) {
  const { t, lang } = useTranslation();

  const isDualSenseExtensionEnabled = status?.dualsense_extension_enabled;
  const isDualSenseConnected = status?.dualsense_connected;

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.overview_title}</h1>
        <p className="panel-desc">{t.overview_desc}</p>
      </div>

      {/* Metrics Grid */}
      <div className="metrics-grid">
        <div className="metric-card">
          <div className="metric-label">{t.metric_steam_status}</div>
          <div className="metric-value text-accent">{t.status_shortcuts_synced}</div>
          <div className="metric-sub">{status?.games_count || 0} games indexed</div>
        </div>

        <div className="metric-card">
          <div className="metric-label">{t.metric_dualsense_status}</div>
          <div
            className={`metric-value ${
              !isDualSenseExtensionEnabled
                ? ''
                : isDualSenseConnected
                  ? 'text-green'
                  : 'text-accent'
            }`}
          >
            {!isDualSenseExtensionEnabled
              ? lang === 'pt'
                ? 'Extensão Desativada'
                : 'Extension Disabled'
              : isDualSenseConnected
                ? lang === 'pt'
                  ? 'Conectado'
                  : 'Connected'
                : lang === 'pt'
                  ? 'Aguardando Controle'
                  : 'Waiting Controller'}
          </div>
          <div className="metric-sub">
            {!isDualSenseExtensionEnabled
              ? lang === 'pt'
                ? 'Ative na aba DualSense Studio'
                : 'Enable in DualSense Studio tab'
              : status?.dualsense_udp_enabled
                ? 'Port 6969 • Link Active'
                : 'Native HID Mode'}
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="section-divider">
        <span className="divider-title">{t.quick_actions_title}</span>
      </div>

      <div className="quick-actions-row">
        <button className="deck-btn deck-btn-primary focusable" onClick={onSyncSteam} tabIndex={10}>
          <Zap size={15} />
          <span>{t.btn_quick_sync_steam}</span>
        </button>
        <button
          className="deck-btn focusable"
          onClick={() => onNavigateTab('dualsense')}
          tabIndex={11}
        >
          <Gamepad2 size={15} />
          <span>{t.tab_dualsense}</span>
        </button>
        <button className="deck-btn focusable" onClick={onScanGames} tabIndex={12}>
          <Search size={15} />
          <span>{t.btn_quick_rescan}</span>
        </button>
      </div>

      {/* Architecture Notice */}
      <div className="info-callout">
        <div className="callout-icon">
          <Info size={20} style={{ color: 'var(--deck-focus)' }} />
        </div>
        <div className="callout-body">
          <strong>{t.callout_philosophy_title}</strong>
          <p>{t.callout_philosophy_body}</p>
        </div>
      </div>
    </section>
  );
}

