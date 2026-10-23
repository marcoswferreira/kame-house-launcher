import { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { SunshineConfig } from '../../../types/models';
import { Save } from 'lucide-react';

interface InputSettingsProps {
  config: SunshineConfig;
  onSaveConfig: (patch: Partial<SunshineConfig>) => void;
}

export function InputSettings({ config, onSaveConfig }: InputSettingsProps) {
  const { t } = useTranslation();
  const [driver, setDriver] = useState(config.gamepad_driver || 'auto');
  const [touchpad, setTouchpad] = useState(config.ds4_back_as_touchpad_click !== 'disabled');
  const [motion, setMotion] = useState(config.motion_as_ds4 !== 'disabled');
  const [port, setPort] = useState(config.port || 47989);
  const [upnp, setUpnp] = useState(config.upnp === 'enabled');

  const handleSave = () => {
    onSaveConfig({
      gamepad_driver: driver === 'auto' ? '' : driver,
      ds4_back_as_touchpad_click: touchpad ? 'enabled' : 'disabled',
      motion_as_ds4: motion ? 'enabled' : 'disabled',
      port,
      upnp: upnp ? 'enabled' : 'disabled',
    });
  };

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2 style={{ fontSize: '16px', fontWeight: 600, color: '#ffffff' }}>{t.input_settings_title}</h2>
        <button className="deck-btn deck-btn-primary focusable" onClick={handleSave}>
          <Save size={14} />
          <span>Save Changes</span>
        </button>
      </div>

      <div className="settings-group">
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_gamepad_driver}</div>
            <div className="setting-sub">{t.setting_gamepad_driver_sub}</div>
          </div>
          <div className="setting-control">
            <select
              className="deck-select focusable"
              value={driver}
              onChange={(e) => setDriver(e.target.value)}
            >
              <option value="auto">Auto-Detect (ViGEmBus / X360)</option>
              <option value="vigem">Force ViGEmBus (Xbox 360 Controller)</option>
              <option value="ds4">Force Sony DualShock 4 (Touchpad + Gyro)</option>
            </select>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_touchpad}</div>
            <div className="setting-sub">{t.setting_touchpad_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={touchpad}
                onChange={(e) => setTouchpad(e.target.checked)}
                className="focusable"
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">Motion / Gyro Sensors</div>
            <div className="setting-sub">Forward device motion controls as DualShock 4 gyro sensors.</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={motion}
                onChange={(e) => setMotion(e.target.checked)}
                className="focusable"
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_sunshine_port}</div>
            <div className="setting-sub">{t.setting_sunshine_port_sub}</div>
          </div>
          <div className="setting-control">
            <input
              type="number"
              className="deck-input focusable"
              style={{ width: '120px' }}
              value={port}
              onChange={(e) => setPort(parseInt(e.target.value, 10))}
            />
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_upnp}</div>
            <div className="setting-sub">{t.setting_upnp_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={upnp}
                onChange={(e) => setUpnp(e.target.checked)}
                className="focusable"
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>
      </div>
    </div>
  );
}

