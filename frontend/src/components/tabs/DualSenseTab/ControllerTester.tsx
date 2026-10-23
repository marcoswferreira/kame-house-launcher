import React, { useState, useEffect } from 'react';
import { useTranslation } from '../../../i18n';
import {
  pollStandardGamepad,
  triggerHapticRumble,
  webHidController,
  LiveGamepadState,
  WebHidDualSenseController,
} from '../../../utils/dualsenseHardware';

interface Props {
  onShowToast?: (message: string) => void;
  isHardwareConnected?: boolean;
  controllerName?: string;
}

export const ControllerTester: React.FC<Props> = ({
  onShowToast,
  isHardwareConnected = false,
  controllerName,
}) => {
  const { t, lang } = useTranslation();
  const [liveGamepad, setLiveGamepad] = useState<LiveGamepadState | null>(null);
  const [isWebHidConnected, setIsWebHidConnected] = useState(webHidController.isConnected());
  const [webHidName, setWebHidName] = useState(webHidController.getDeviceName());
  const isWebHidAvailable = WebHidDualSenseController.isSupported();

  useEffect(() => {
    let animId: number;

    const poll = () => {
      const state = pollStandardGamepad();
      setLiveGamepad(state);
      setIsWebHidConnected(webHidController.isConnected());
      setWebHidName(webHidController.getDeviceName());
      animId = requestAnimationFrame(poll);
    };

    animId = requestAnimationFrame(poll);
    return () => cancelAnimationFrame(animId);
  }, []);

  const handleConnectWebHid = async () => {
    const success = await webHidController.connect();
    if (success) {
      setIsWebHidConnected(true);
      setWebHidName(webHidController.getDeviceName());
      onShowToast?.(t('toast_webhid_connected'));
    } else {
      onShowToast?.(t('toast_webhid_failed'));
    }
  };

  const handleDisconnectWebHid = async () => {
    await webHidController.disconnect();
    setIsWebHidConnected(false);
    onShowToast?.(t('toast_webhid_disconnected'));
  };

  const handleTestRumble = async () => {
    const success = await triggerHapticRumble(500, 0.9, 0.9);
    if (success) {
      onShowToast?.(t('toast_rumble_tested'));
    } else {
      onShowToast?.(t('toast_rumble_failed'));
    }
  };

  return (
    <div className="controller-tester-container">
      {/* Top Banner: Detection & Hardware Connect */}
      <div className="deck-card tester-status-banner">
        <div className="banner-info">
          <h4>🕹️ {t('dualsense_tester_title')}</h4>
          <p className="section-subtitle">{t('dualsense_tester_subtitle')}</p>
          <div className="status-badges">
            <span
              className={`badge-tag ${
                liveGamepad?.connected || isHardwareConnected ? 'live-badge' : 'danger'
              }`}
            >
              {liveGamepad?.connected
                ? `🟢 Gamepad: ${liveGamepad.id}`
                : isHardwareConnected
                  ? `🟢 ${controllerName || 'Sony DualSense Wireless Controller'} (${
                      lang === 'pt'
                        ? 'Hardware Ativo — Pressione um botão para testar na tela'
                        : 'Hardware Active — Press any button to test on screen'
                    })`
                  : `⚠️ ${t('dualsense_tester_no_gamepad')}`}
            </span>
            {isWebHidConnected ? (
              <span className="badge-tag live-badge">
                ⚡ WebHID Hardware: {webHidName}
              </span>
            ) : (
              <span className="badge-tag">
                ⚡ WebHID: {isWebHidAvailable ? t('dualsense_webhid_standby') : t('dualsense_webhid_unsupported')}
              </span>
            )}
          </div>
        </div>

        <div className="banner-actions">
          {isWebHidAvailable && (
            isWebHidConnected ? (
              <button className="deck-btn danger" onClick={handleDisconnectWebHid}>
                🔌 {t('dualsense_disconnect_webhid')}
              </button>
            ) : (
              <button className="deck-btn primary" onClick={handleConnectWebHid}>
                ⚡ {t('dualsense_connect_webhid')}
              </button>
            )
          )}
          <button className="deck-btn secondary" onClick={handleTestRumble}>
            📳 {t('dualsense_test_rumble')}
          </button>
        </div>
      </div>

      {/* Controller Interactive Visualizer */}
      <div className="tester-grid">
        {/* Analog Sticks & Triggers */}
        <div className="deck-card stick-card">
          <h4>🕹️ {t('dualsense_tester_analog_sticks')}</h4>
          <div className="sticks-row">
            {/* Left Stick */}
            <div className="stick-box">
              <span className="stick-label">L3 (Left Stick)</span>
              <div className={`stick-visualizer ${liveGamepad?.buttons.l3 ? 'active' : ''}`}>
                <div
                  className="stick-thumb"
                  style={{
                    transform: `translate(${((liveGamepad?.leftStick.x || 0) * 35)}px, ${((liveGamepad?.leftStick.y || 0) * 35)}px)`,
                  }}
                />
              </div>
              <div className="stick-coords">
                X: {liveGamepad?.leftStick.x?.toFixed(2) || '0.00'} | Y: {liveGamepad?.leftStick.y?.toFixed(2) || '0.00'}
              </div>
            </div>

            {/* Right Stick */}
            <div className="stick-box">
              <span className="stick-label">R3 (Right Stick)</span>
              <div className={`stick-visualizer ${liveGamepad?.buttons.r3 ? 'active' : ''}`}>
                <div
                  className="stick-thumb"
                  style={{
                    transform: `translate(${((liveGamepad?.rightStick.x || 0) * 35)}px, ${((liveGamepad?.rightStick.y || 0) * 35)}px)`,
                  }}
                />
              </div>
              <div className="stick-coords">
                X: {liveGamepad?.rightStick.x?.toFixed(2) || '0.00'} | Y: {liveGamepad?.rightStick.y?.toFixed(2) || '0.00'}
              </div>
            </div>
          </div>

          {/* Trigger Gauges */}
          <div className="trigger-gauges">
            <div className="gauge-group">
              <div className="gauge-header">
                <span>L2 Trigger</span>
                <span>{liveGamepad?.l2 || 0}%</span>
              </div>
              <div className="gauge-bar">
                <div
                  className="gauge-fill"
                  style={{ width: `${liveGamepad?.l2 || 0}%` }}
                />
              </div>
            </div>

            <div className="gauge-group">
              <div className="gauge-header">
                <span>R2 Trigger</span>
                <span>{liveGamepad?.r2 || 0}%</span>
              </div>
              <div className="gauge-bar">
                <div
                  className="gauge-fill"
                  style={{ width: `${liveGamepad?.r2 || 0}%` }}
                />
              </div>
            </div>
          </div>
        </div>

        {/* Buttons & D-Pad */}
        <div className="deck-card buttons-card">
          <h4>🎮 {t('dualsense_tester_buttons')}</h4>

          <div className="buttons-layout">
            {/* D-Pad */}
            <div className="dpad-cluster">
              <span className="cluster-label">D-PAD</span>
              <div className="dpad-cross">
                <div className={`dpad-btn up ${liveGamepad?.buttons.dpadUp ? 'active' : ''}`}>▲</div>
                <div className="dpad-mid">
                  <div className={`dpad-btn left ${liveGamepad?.buttons.dpadLeft ? 'active' : ''}`}>◀</div>
                  <div className="dpad-center" />
                  <div className={`dpad-btn right ${liveGamepad?.buttons.dpadRight ? 'active' : ''}`}>▶</div>
                </div>
                <div className={`dpad-btn down ${liveGamepad?.buttons.dpadDown ? 'active' : ''}`}>▼</div>
              </div>
            </div>

            {/* Action Buttons */}
            <div className="actions-cluster">
              <span className="cluster-label">ACTION BUTTONS</span>
              <div className="action-cross">
                <div className={`action-btn triangle ${liveGamepad?.buttons.triangle ? 'active' : ''}`}>▲</div>
                <div className="action-mid">
                  <div className={`action-btn square ${liveGamepad?.buttons.square ? 'active' : ''}`}>◼</div>
                  <div className="action-center" />
                  <div className={`action-btn circle ${liveGamepad?.buttons.circle ? 'active' : ''}`}>●</div>
                </div>
                <div className={`action-btn cross ${liveGamepad?.buttons.cross ? 'active' : ''}`}>✖</div>
              </div>
            </div>
          </div>

          {/* Shoulder & System Buttons */}
          <div className="system-buttons-row">
            <span className={`key-badge ${liveGamepad?.buttons.l1 ? 'active' : ''}`}>L1</span>
            <span className={`key-badge ${liveGamepad?.buttons.r1 ? 'active' : ''}`}>R1</span>
            <span className={`key-badge ${liveGamepad?.buttons.create ? 'active' : ''}`}>SHARE / CREATE</span>
            <span className={`key-badge ${liveGamepad?.buttons.options ? 'active' : ''}`}>OPTIONS</span>
            <span className={`key-badge ${liveGamepad?.buttons.ps ? 'active' : ''}`}>PS</span>
            <span className={`key-badge ${liveGamepad?.buttons.touchpad ? 'active' : ''}`}>TOUCHPAD</span>
          </div>
        </div>
      </div>
    </div>
  );
};

