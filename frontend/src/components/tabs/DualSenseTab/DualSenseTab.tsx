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
import { webHidController } from '../../../utils/dualsenseHardware';

type SubTab = 'tester' | 'triggers' | 'lighting' | 'emulation' | 'udp' | 'profiles';

interface Props {
  onShowToast?: (message: string) => void;
}

export const DualSenseTab: React.FC<Props> = ({ onShowToast }) => {
  const { t } = useTranslation();
  const [activeSubTab, setActiveSubTab] = useState<SubTab>('tester');
  const [status, setStatus] = useState<DualSenseStatus | null>(null);
  const [loading, setLoading] = useState(true);

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

  if (loading && !status) {
    return (
      <div className="tab-pane-content loading-pane">
        <p>{t('loading')}...</p>
      </div>
    );
  }

  const currentStatus = status || {
    connected: false,
    name: 'Sony DualSense Wireless Controller',
    connection_type: 'usb' as const,
    battery: { percentage: 90, is_charging: false, is_full: false },
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
      running: true,
      port: 6969,
      packets_received: 0,
      last_packet_time: null,
      last_sender: null,
    },
    profiles: [],
    mic_muted: false,
    audio_device_detected: true,
    speaker_volume: 80,
  };

  return (
    <div className="tab-pane-content dualsense-pane">
      {/* Controller Master Top Bar */}
      <div className="dualsense-master-bar deck-card">
        <div className="master-bar-left">
          <div className="controller-icon-badge">🎮</div>
          <div>
            <h2 className="master-title">{currentStatus.name}</h2>
            <div className="master-subtitle">
              <span className="badge-tag live-badge">
                {currentStatus.connected ? t('dualsense_connected') : t('dualsense_disconnected')}
              </span>
              <span className="badge-tag">
                {currentStatus.connection_type.toUpperCase()}
              </span>
              <span className="badge-tag">
                🔋 {currentStatus.battery.percentage}%
              </span>
            </div>
          </div>
        </div>

        <div className="master-bar-right">
          <button
            className={`deck-btn ${currentStatus.mic_muted ? 'danger' : 'secondary'}`}
            onClick={handleToggleMic}
            title={t('btn_toggle_mic')}
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
          <ControllerTester onShowToast={onShowToast} />
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
    </div>
  );
};

