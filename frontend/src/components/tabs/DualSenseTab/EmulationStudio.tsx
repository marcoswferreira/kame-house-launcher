import React, { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { EmulationConfig, EmulationMode } from '../../../types/models';

interface Props {
  emulation: EmulationConfig;
  onApply: (config: EmulationConfig) => void;
}

export const EmulationStudio: React.FC<Props> = ({ emulation: initialEmulation, onApply }) => {
  const { t } = useTranslation();
  const [emulation, setEmulation] = useState<EmulationConfig>(initialEmulation);

  const setMode = (mode: EmulationMode) => {
    const updated = { ...emulation, mode };
    setEmulation(updated);
    onApply(updated);
  };

  const toggleGyro = () => {
    const updated = {
      ...emulation,
      gyro: { ...emulation.gyro, enabled: !emulation.gyro.enabled },
    };
    setEmulation(updated);
    onApply(updated);
  };

  const updateGyro = (patch: Partial<EmulationConfig['gyro']>) => {
    const updated = {
      ...emulation,
      gyro: { ...emulation.gyro, ...patch },
    };
    setEmulation(updated);
    onApply(updated);
  };

  return (
    <div className="emulation-studio-container">
      <div className="studio-header">
        <div>
          <h3>{t('dualsense_emulation_title')}</h3>
          <p className="section-subtitle">{t('dualsense_emulation_subtitle')}</p>
        </div>
      </div>

      <div className="emulation-modes-grid">
        {/* Native Mode */}
        <div
          className={`deck-card emulation-mode-card ${emulation.mode === 'native' ? 'selected' : ''}`}
          onClick={() => setMode('native')}
        >
          <div className="card-top-icon">🎮</div>
          <h4>{t('dualsense_mode_native')}</h4>
          <p>{t('dualsense_mode_native_desc')}</p>
          <span className="badge-tag">{emulation.mode === 'native' ? 'ACTIVE' : 'SELECT'}</span>
        </div>

        {/* Xbox 360 / XInput */}
        <div
          className={`deck-card emulation-mode-card ${emulation.mode === 'xbox360' ? 'selected' : ''}`}
          onClick={() => setMode('xbox360')}
        >
          <div className="card-top-icon">🟢</div>
          <h4>{t('dualsense_mode_xbox360')}</h4>
          <p>{t('dualsense_mode_xbox360_desc')}</p>
          <span className="badge-tag">{emulation.mode === 'xbox360' ? 'ACTIVE' : 'SELECT'}</span>
        </div>

        {/* DualShock 4 */}
        <div
          className={`deck-card emulation-mode-card ${emulation.mode === 'dualshock4' ? 'selected' : ''}`}
          onClick={() => setMode('dualshock4')}
        >
          <div className="card-top-icon">🔵</div>
          <h4>{t('dualsense_mode_ds4')}</h4>
          <p>{t('dualsense_mode_ds4_desc')}</p>
          <span className="badge-tag">{emulation.mode === 'dualshock4' ? 'ACTIVE' : 'SELECT'}</span>
        </div>
      </div>

      {/* Gyro Aiming Station */}
      <div className="deck-card gyro-card">
        <div className="gyro-header">
          <div>
            <h4>🎯 {t('dualsense_gyro_title')}</h4>
            <p className="section-subtitle">{t('dualsense_gyro_subtitle')}</p>
          </div>
          <button
            className={`deck-btn ${emulation.gyro.enabled ? 'danger' : 'primary'}`}
            onClick={toggleGyro}
          >
            {emulation.gyro.enabled ? t('dualsense_gyro_disable') : t('dualsense_gyro_enable')}
          </button>
        </div>

        {emulation.gyro.enabled && (
          <div className="gyro-controls-grid">
            <div className="form-group">
              <label>{t('dualsense_gyro_sensitivity')}: {emulation.gyro.sensitivity.toFixed(1)}x</label>
              <input
                type="range"
                min="0.5"
                max="3.0"
                step="0.1"
                value={emulation.gyro.sensitivity}
                onChange={(e) => updateGyro({ sensitivity: parseFloat(e.target.value) })}
              />
            </div>

            <div className="form-group">
              <label>{t('dualsense_gyro_deadzone')}: {emulation.gyro.deadzone}</label>
              <input
                type="range"
                min="0"
                max="150"
                step="5"
                value={emulation.gyro.deadzone}
                onChange={(e) => updateGyro({ deadzone: parseInt(e.target.value, 10) })}
              />
            </div>

            <div className="checkbox-row">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={emulation.gyro.invert_x}
                  onChange={(e) => updateGyro({ invert_x: e.target.checked })}
                />
                {t('dualsense_gyro_invert_x')}
              </label>
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={emulation.gyro.invert_y}
                  onChange={(e) => updateGyro({ invert_y: e.target.checked })}
                />
                {t('dualsense_gyro_invert_y')}
              </label>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

