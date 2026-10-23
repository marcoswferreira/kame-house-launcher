import { useTranslation } from '../../i18n';
import { StatusResponse } from '../../types/models';
import { Zap, KeyRound, Search, UploadCloud, Info } from 'lucide-react';

interface OverviewTabProps {
  status: StatusResponse | null;
  onNavigateTab: (tab: string) => void;
  onSyncSteam: () => void;
  onScanGames: () => void;
  onSyncSunshine: () => void;
}

export function OverviewTab({
  status,
  onNavigateTab,
  onSyncSteam,
  onScanGames,
  onSyncSunshine,
}: OverviewTabProps) {
  const { t, lang } = useTranslation();

  const isSunshineActive = status?.sunshine_enabled;

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
          <div className="metric-label">{t.metric_sunshine_status}</div>
          <div className={`metric-value ${isSunshineActive ? 'text-green' : ''}`}>
            {isSunshineActive ? 'Online' : 'Disabled'}
          </div>
          <div className="metric-sub">
            {isSunshineActive ? 'Port 47990 • HTTPS Ready' : 'Opt-In Standby'}
          </div>
        </div>

        <div className="metric-card">
          <div className="metric-label">{t.metric_tv_profile}</div>
          <div className="metric-value">4K @ 120Hz</div>
          <div className="metric-sub">{t.metric_hdr_active}</div>
        </div>

        <div className="metric-card">
          <div className="metric-label">{t.metric_launch_supervisor}</div>
          <div className="metric-value text-accent">
            {status?.launch_behavior === 'keep_open'
              ? (lang === 'pt' ? 'Segundo Plano' : 'Keep Open')
              : t.mode_hide_restore}
          </div>
          <div className="metric-sub">{t.mode_restore_sub}</div>
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
          onClick={() => onNavigateTab('sunshine')}
          tabIndex={11}
        >
          <KeyRound size={15} />
          <span>{t.btn_quick_pair_pin}</span>
        </button>
        <button className="deck-btn focusable" onClick={onScanGames} tabIndex={12}>
          <Search size={15} />
          <span>{t.btn_quick_rescan}</span>
        </button>
        <button className="deck-btn focusable" onClick={onSyncSunshine} tabIndex={13}>
          <UploadCloud size={15} />
          <span>{t.btn_quick_sync_sunshine}</span>
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

