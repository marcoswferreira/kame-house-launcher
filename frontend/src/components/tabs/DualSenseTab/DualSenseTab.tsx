import React, { useState, useEffect, useCallback } from 'react';
import { useTranslation } from '../../../i18n';
import {
  DualSenseStatus,
  DualTriggerConfig,
  LightingConfig,
  EmulationConfig,
} from '../../../types/models';
import { TriggerStudio } from './TriggerStudio';
import { LightingStudio } from './LightingStudio';
import { EmulationStudio } from './EmulationStudio';
import { UdpServerStudio } from './UdpServerStudio';
import { GameProfilesStudio } from './GameProfilesStudio';
import { ControllerTester } from './ControllerTester';
import { webHidController, WebHidDualSenseController } from '../../../utils/dualsenseHardware';

type SubTab = 'tester' | 'triggers' | 'lighting' | 'emulation' | 'udp' | 'profiles';

interface Props {
  onShowToast?: (message: string) => void;
  onRefreshAppStatus?: () => void;
}

export const DualSenseTab: React.FC<Props> = ({ onShowToast, onRefreshAppStatus }) => {
  const { t, lang } = useTranslation();
  const [activeSubTab, setActiveSubTab] = useState<SubTab>('tester');
  const [status, setStatus] = useState<DualSenseStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [isWebHidConnected, setIsWebHidConnected] = useState(webHidController.isConnected());

  useEffect(() => {
    const timer = setInterval(() => {
      setIsWebHidConnected(webHidController.isConnected());
    }, 800);
    return () => clearInterval(timer);
  }, []);

  const handleConnectWebHid = async () => {
    const ok = await webHidController.connect();
    setIsWebHidConnected(ok);
    if (ok) {
      onShowToast?.(t('toast_webhid_connected'));
      if (status) {
        await webHidController.sendReport(status.triggers, status.lighting);
      }
    } else {
      onShowToast?.(t('toast_webhid_failed'));
    }
  };

  const handleDisconnectWebHid = async () => {
    await webHidController.disconnect();
    setIsWebHidConnected(false);
    onShowToast?.(t('toast_webhid_disconnected'));
  };

  const fetchStatus = useCallback(async () => {
    try {
      const res = await fetch('/api/dualsense/status');
      if (res.ok) {
        const data: DualSenseStatus = await res.json();
        setStatus(data);
      }
    } catch (e) {
      console.error('Failed to load DualSense status', e);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 3000);
    return () => clearInterval(interval);
  }, [fetchStatus]);

  const handleApplyTriggers = async (triggers: DualTriggerConfig) => {
    try {
      if (webHidController.isConnected()) {
        const currentLighting = status?.lighting || {
          lightbar: { mode: 'static' as const, color: { r: 0, g: 68, b: 204 } },
          player_led: 'player1' as const,
          mic_led: 'off' as const,
        };
        await webHidController.sendReport(triggers, currentLighting);
      }

      const res = await fetch('/api/dualsense/trigger', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(triggers),
      });
      if (res.ok) {
        onShowToast?.(t('toast_dualsense_triggers_applied'));
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleApplyLighting = async (lighting: LightingConfig) => {
    try {
      if (webHidController.isConnected()) {
        const currentTriggers = status?.triggers || {
          left: { mode: 'off' as const },
          right: { mode: 'off' as const },
        };
        await webHidController.sendReport(currentTriggers, lighting);
      }

      const res = await fetch('/api/dualsense/lightbar', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(lighting),
      });
      if (res.ok) {
        onShowToast?.(t('toast_dualsense_lighting_applied'));
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleApplyEmulation = async (emulation: EmulationConfig) => {
    try {
      const res = await fetch('/api/dualsense/emulation', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(emulation),
      });
      if (res.ok) {
        onShowToast?.(t('toast_dualsense_emulation_applied'));
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleToggleUdp = async (enabled: boolean) => {
    try {
      const res = await fetch('/api/dualsense/udp/toggle', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled }),
      });
      if (res.ok) {
        onShowToast?.(
          enabled ? t('toast_dualsense_udp_started') : t('toast_dualsense_udp_stopped')
        );
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleToggleMic = async () => {
    try {
      const res = await fetch('/api/dualsense/mic/toggle', { method: 'POST' });
      if (res.ok) {
        const data = await res.json();
        onShowToast?.(data.mic_muted ? t('toast_mic_muted') : t('toast_mic_unmuted'));
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleApplyProfile = async (profileId: string) => {
    try {
      const res = await fetch('/api/dualsense/profile', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: profileId }),
      });
      if (res.ok) {
        onShowToast?.(t('toast_dualsense_profile_applied'));
        fetchStatus();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleToggleExtension = async (enabled: boolean) => {
    try {
      const res = await fetch('/api/dualsense/extension/toggle', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled }),
      });
      if (res.ok) {
        onShowToast?.(
          enabled
            ? lang === 'pt'
              ? 'Extensão DualSense Studio ativada!'
              : 'DualSense Studio extension enabled!'
            : lang === 'pt'
              ? 'Extensão DualSense Studio desativada.'
              : 'DualSense Studio extension disabled.'
        );
        await fetchStatus();
        onRefreshAppStatus?.();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  const handleToggleAutostart = async (autostart: boolean) => {
    try {
      const res = await fetch('/api/dualsense/extension/toggle', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ autostart }),
      });
      if (res.ok) {
        onShowToast?.(
          autostart
            ? lang === 'pt'
              ? 'Inicialização automática da extensão ativada.'
              : 'Extension autostart on launch enabled.'
            : lang === 'pt'
              ? 'Inicialização automática da extensão desativada.'
              : 'Extension autostart on launch disabled.'
        );
        await fetchStatus();
        onRefreshAppStatus?.();
      }
    } catch {
      onShowToast?.(t('toast_error'));
    }
  };

  if (loading && !status) {
    return (
      <div className="tab-pane-content loading-pane">
        <p>{t('loading')}...</p>
      </div>
    );
  }

  const currentStatus: DualSenseStatus = status || {
    extension_enabled: false,
    autostart_on_launch: false,
    connected: false,
    name: 'Disconnected',
    connection_type: 'usb' as const,
    battery: { percentage: 0, is_charging: false, is_full: false },
    triggers: { left: { mode: 'off' as const }, right: { mode: 'off' as const } },
    lighting: {
      lightbar: { mode: 'static' as const, color: { r: 0, g: 68, b: 204 } },
      player_led: 'player1' as const,
      mic_led: 'off' as const,
    },
    emulation: {
      mode: 'native' as const,
      gyro: { enabled: false, sensitivity: 1.0, deadzone: 50, invert_x: false, invert_y: false },
    },
    udp_stats: {
      running: false,
      port: 6969,
      packets_received: 0,
      last_packet_time: null,
      last_sender: null,
    },
    profiles: [],
    mic_muted: false,
    audio_device_detected: false,
    speaker_volume: 80,
  };

  const isHardwareConnected = currentStatus.connected || isWebHidConnected;
  const displayControllerName = currentStatus.connected
    ? currentStatus.name
    : isWebHidConnected
      ? 'Sony DualSense Wireless Controller (WebHID)'
      : lang === 'pt'
        ? 'Nenhum Controle Conectado'
        : 'No Controller Connected';

  return (
    <div className="tab-pane-content dualsense-pane">
      {/* DualSense Extension Management Header */}
      <div
        className="deck-card"
        style={{
          marginBottom: '16px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          flexWrap: 'wrap',
          gap: '16px',
          borderLeft: currentStatus.extension_enabled
            ? '4px solid #3b82f6'
            : '4px solid rgba(255, 255, 255, 0.2)',
        }}
      >
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '4px' }}>
            <h3 style={{ margin: 0, fontSize: '16px', color: '#fff' }}>
              🧩 {lang === 'pt' ? 'Extensão DualSense Studio' : 'DualSense Studio Extension'}
            </h3>
            <span
              className={`badge-tag ${currentStatus.extension_enabled ? 'live-badge' : ''}`}
              style={
                !currentStatus.extension_enabled
                  ? { background: 'rgba(255,255,255,0.08)', color: '#9ca3af' }
                  : undefined
              }
            >
              {currentStatus.extension_enabled
                ? lang === 'pt'
                  ? 'ATIVA'
                  : 'ENABLED'
                : lang === 'pt'
                  ? 'DESATIVADA'
                  : 'DISABLED'}
            </span>
          </div>
          <p style={{ margin: 0, fontSize: '13px', color: 'var(--deck-text-muted, #9ca3af)' }}>
            {lang === 'pt'
              ? 'Esta extensão gerencia gatilhos adaptáveis, Lightbar RGB, telemetria HID e servidor UDP.'
              : 'This extension manages adaptive triggers, RGB Lightbar, HID telemetry, and UDP server.'}
          </p>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '16px', flexWrap: 'wrap' }}>
          <label
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              fontSize: '13px',
              color: '#e5e7eb',
              cursor: 'pointer',
              userSelect: 'none',
            }}
          >
            <input
              type="checkbox"
              checked={currentStatus.autostart_on_launch}
              onChange={(e) => handleToggleAutostart(e.target.checked)}
              style={{ width: '16px', height: '16px', accentColor: '#3b82f6', cursor: 'pointer' }}
            />
            {lang === 'pt'
              ? 'Iniciar automaticamente junto com a aplicação'
              : 'Start automatically with the application'}
          </label>

          <button
            className={`deck-btn ${currentStatus.extension_enabled ? 'danger' : 'primary'}`}
            onClick={() => handleToggleExtension(!currentStatus.extension_enabled)}
          >
            {currentStatus.extension_enabled
              ? lang === 'pt'
                ? '⏻ Desativar Extensão'
                : '⏻ Disable Extension'
              : lang === 'pt'
                ? '⚡ Ativar Extensão DualSense'
                : '⚡ Enable DualSense Extension'}
          </button>
        </div>
      </div>

      {!currentStatus.extension_enabled ? (
        <div
          className="deck-card"
          style={{
            textAlign: 'center',
            padding: '48px 24px',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            gap: '14px',
          }}
        >
          <div style={{ fontSize: '42px', opacity: 0.6 }}>🎮</div>
          <h2 style={{ margin: 0, fontSize: '20px', color: '#fff' }}>
            {lang === 'pt'
              ? 'A Extensão DualSense Studio está Desligada'
              : 'DualSense Studio Extension is Turned Off'}
          </h2>
          <p
            style={{
              margin: 0,
              maxWidth: '540px',
              fontSize: '14px',
              lineHeight: 1.6,
              color: 'var(--deck-text-muted, #9ca3af)',
            }}
          >
            {lang === 'pt'
              ? 'Por padrão, a extensão DualSense permanece desligada para não consumir recursos em segundo plano nem monitorar portas USB/Bluetooth. Clique no botão abaixo para ativá-la agora, ou marque a opção acima caso deseje que ela inicie junto com o Steam Companion.'
              : 'By default, the DualSense extension remains turned off so it does not consume background resources or poll USB/Bluetooth ports. Click the button below to enable it now, or check the option above if you want it to start automatically with Steam Companion.'}
          </p>
          <button
            className="deck-btn primary"
            onClick={() => handleToggleExtension(true)}
            style={{ marginTop: '8px', padding: '10px 24px', fontSize: '14px' }}
          >
            {lang === 'pt' ? '⚡ Ativar Extensão Agora' : '⚡ Enable Extension Now'}
          </button>
        </div>
      ) : (
        <>
          {/* Controller Master Top Bar */}
          <div className="dualsense-master-bar deck-card">
            <div className="master-bar-left">
              <div className="controller-icon-badge" style={{ opacity: isHardwareConnected ? 1 : 0.45 }}>
                🎮
              </div>
              <div>
                <h2 className="master-title">{displayControllerName}</h2>
                <div className="master-subtitle">
                  <span
                    className={`badge-tag ${isHardwareConnected ? 'live-badge' : ''}`}
                    style={
                      !isHardwareConnected
                        ? { background: 'rgba(239, 68, 68, 0.15)', color: '#f87171', border: '1px solid rgba(239, 68, 68, 0.3)' }
                        : undefined
                    }
                  >
                    {isHardwareConnected ? t('dualsense_connected') : t('dualsense_disconnected')}
                  </span>
                  {currentStatus.connected && (
                    <>
                      <span className="badge-tag">
                        {currentStatus.connection_type.toUpperCase()}
                      </span>
                      <span className="badge-tag">
                        🔋 {currentStatus.battery.percentage}%
                        {currentStatus.battery.is_charging ? ' ⚡' : ''}
                      </span>
                    </>
                  )}
                  {!currentStatus.connected && isWebHidConnected && (
                    <span className="badge-tag">WEBHID</span>
                  )}
                </div>
              </div>
            </div>

            <div className="master-bar-right">
              {WebHidDualSenseController.isSupported() && (
                <button
                  className={`deck-btn ${isWebHidConnected ? 'primary' : 'secondary'}`}
                  onClick={isWebHidConnected ? handleDisconnectWebHid : handleConnectWebHid}
                  title={isWebHidConnected ? t('dualsense_disconnect_webhid') : t('dualsense_connect_webhid')}
                >
                  {isWebHidConnected ? '⚡ WebHID [ATIVO]' : '⚡ Conectar WebHID'}
                </button>
              )}
              <button
                className={`deck-btn ${currentStatus.mic_muted ? 'danger' : 'secondary'}`}
                onClick={handleToggleMic}
                title={t('btn_toggle_mic')}
                disabled={!isHardwareConnected}
              >
                {currentStatus.mic_muted ? '🎙️ MIC [MUTED]' : '🎙️ MIC [ACTIVE]'}
              </button>
              <button
                className="deck-btn secondary"
                onClick={() => handleToggleUdp(!currentStatus.udp_stats.running)}
              >
                {currentStatus.udp_stats.running
                  ? `📡 UDP :${currentStatus.udp_stats.port} [ON]`
                  : `📡 UDP :${currentStatus.udp_stats.port} [OFF]`}
              </button>
            </div>
          </div>

          {/* Sub-Navigation Pills */}
          <div className="dualsense-subnav">
            <button
              className={`subnav-pill ${activeSubTab === 'tester' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('tester')}
            >
              🕹️ {t('dualsense_subtab_tester')}
            </button>
            <button
              className={`subnav-pill ${activeSubTab === 'triggers' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('triggers')}
            >
              ⚡ {t('dualsense_subtab_triggers')}
            </button>
            <button
              className={`subnav-pill ${activeSubTab === 'lighting' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('lighting')}
            >
              💡 {t('dualsense_subtab_lighting')}
            </button>
            <button
              className={`subnav-pill ${activeSubTab === 'emulation' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('emulation')}
            >
              🎮 {t('dualsense_subtab_emulation')}
            </button>
            <button
              className={`subnav-pill ${activeSubTab === 'udp' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('udp')}
            >
              📡 {t('dualsense_subtab_udp')}
            </button>
            <button
              className={`subnav-pill ${activeSubTab === 'profiles' ? 'active' : ''}`}
              onClick={() => setActiveSubTab('profiles')}
            >
              📁 {t('dualsense_subtab_profiles')}
            </button>
          </div>

          {/* Active SubTab View */}
          <div className="dualsense-subview-area">
            {activeSubTab === 'tester' && (
              <ControllerTester
                onShowToast={onShowToast}
                isHardwareConnected={isHardwareConnected}
                controllerName={displayControllerName}
              />
            )}
            {activeSubTab === 'triggers' && (
              <TriggerStudio
                triggers={currentStatus.triggers}
                onApply={handleApplyTriggers}
              />
            )}
            {activeSubTab === 'lighting' && (
              <LightingStudio
                lighting={currentStatus.lighting}
                onApply={handleApplyLighting}
              />
            )}
            {activeSubTab === 'emulation' && (
              <EmulationStudio
                emulation={currentStatus.emulation}
                onApply={handleApplyEmulation}
              />
            )}
            {activeSubTab === 'udp' && (
              <UdpServerStudio
                stats={currentStatus.udp_stats}
                onToggle={handleToggleUdp}
              />
            )}
            {activeSubTab === 'profiles' && (
              <GameProfilesStudio
                profiles={currentStatus.profiles}
                onApplyProfile={handleApplyProfile}
              />
            )}
          </div>
        </>
      )}
    </div>
  );
};

