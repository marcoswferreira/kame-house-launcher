import { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { SunshineConfig, SunshineApp, SunshineClientDevice } from '../../../types/models';
import { StandbyView } from './StandbyView';
import { PairingStation } from './PairingStation';
import { StreamedAppsManager } from './StreamedAppsManager';
import { AVSettings } from './AVSettings';
import { InputSettings } from './InputSettings';
import { LogTerminal } from './LogTerminal';
import { KeyRound, LayoutGrid, Sliders, Gamepad2, Terminal, PowerOff, ExternalLink } from 'lucide-react';

interface SunshineTabProps {
  sunshineEnabled: boolean;
  onToggleSunshine: (enabled: boolean) => void;
  config: SunshineConfig;
  apps: SunshineApp[];
  clients: SunshineClientDevice[];
  logs: string;
  onPairPin: (pin: string) => Promise<boolean>;
  onUnpairClient: (uuid: string) => void;
  onUnpairAll: () => void;
  onSyncAppsJson: () => void;
  onSaveApp: (app: SunshineApp) => void;
  onDeleteApp: (index: number) => void;
  onSaveConfig: (patch: Partial<SunshineConfig>) => void;
  onRefreshLogs: () => void;
  onRestartSunshine: () => void;
  onResetDisplay: () => void;
}

export function SunshineTab({
  sunshineEnabled,
  onToggleSunshine,
  config,
  apps,
  clients,
  logs,
  onPairPin,
  onUnpairClient,
  onUnpairAll,
  onSyncAppsJson,
  onSaveApp,
  onDeleteApp,
  onSaveConfig,
  onRefreshLogs,
  onRestartSunshine,
  onResetDisplay,
}: SunshineTabProps) {
  const { t } = useTranslation();
  const [activeSubTab, setActiveSubTab] = useState<'pairing' | 'apps' | 'av' | 'input' | 'logs'>('pairing');

  // If Sunshine is disabled, render non-intrusive Standby view
  if (!sunshineEnabled) {
    return (
      <section className="tab-panel">
        <StandbyView onEnableSunshine={() => onToggleSunshine(true)} />
      </section>
    );
  }

  const subTabs = [
    { id: 'pairing', label: t.sunshine_subtab_pairing, icon: KeyRound },
    { id: 'apps', label: t.sunshine_subtab_apps, icon: LayoutGrid },
    { id: 'av', label: t.sunshine_subtab_av, icon: Sliders },
    { id: 'input', label: t.sunshine_subtab_input, icon: Gamepad2 },
    { id: 'logs', label: t.sunshine_subtab_logs, icon: Terminal },
  ] as const;

  return (
    <section className="tab-panel">
      {/* Sunshine Active Master Bar */}
      <div className="sunshine-master-bar">
        <div className="master-bar-left">
          <span className="chip-dot" />
          <span style={{ fontWeight: 700, fontSize: '13px', color: '#ffffff' }}>
            SUNSHINE DAEMON ACTIVE (:47990)
          </span>
          <a
            href="https://localhost:47990"
            target="_blank"
            rel="noreferrer"
            className="hud-pill-btn"
            style={{ textDecoration: 'none' }}
          >
            <ExternalLink size={12} />
            <span>Open Web UI ↗</span>
          </a>
        </div>

        <button
          className="deck-btn deck-btn-danger focusable"
          style={{ padding: '6px 12px', fontSize: '11px' }}
          onClick={() => onToggleSunshine(false)}
        >
          <PowerOff size={12} />
          <span>{t.btn_disable_sunshine}</span>
        </button>
      </div>

      {/* Sub-Navigation Pills */}
      <div className="sunshine-subnav">
        {subTabs.map((st) => {
          const Icon = st.icon;
          const isActive = activeSubTab === st.id;
          return (
            <button
              key={st.id}
              className={`subnav-btn focusable ${isActive ? 'active' : ''}`}
              onClick={() => setActiveSubTab(st.id)}
            >
              <Icon size={14} />
              <span>{st.label}</span>
            </button>
          );
        })}
      </div>

      {/* Active Sub-Tab View */}
      {activeSubTab === 'pairing' && (
        <PairingStation
          clients={clients}
          onPairPin={onPairPin}
          onUnpairClient={onUnpairClient}
          onUnpairAll={onUnpairAll}
        />
      )}

      {activeSubTab === 'apps' && (
        <StreamedAppsManager
          apps={apps}
          onSyncAppsJson={onSyncAppsJson}
          onSaveApp={onSaveApp}
          onDeleteApp={onDeleteApp}
        />
      )}

      {activeSubTab === 'av' && (
        <AVSettings config={config} onSaveConfig={onSaveConfig} />
      )}

      {activeSubTab === 'input' && (
        <InputSettings config={config} onSaveConfig={onSaveConfig} />
      )}

      {activeSubTab === 'logs' && (
        <LogTerminal
          logs={logs}
          onRefreshLogs={onRefreshLogs}
          onRestartSunshine={onRestartSunshine}
          onResetDisplay={onResetDisplay}
        />
      )}
    </section>
  );
}

