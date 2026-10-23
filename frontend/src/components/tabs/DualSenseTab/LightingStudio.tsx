import React, { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { LightingConfig, LightbarMode, PlayerLed, MicLed, RgbColor } from '../../../types/models';

interface Props {
  lighting: LightingConfig;
  onApply: (lighting: LightingConfig) => void;
}

const COLOR_PRESETS: { label: string; color: RgbColor }[] = [
  { label: 'Sony Blue', color: { r: 0, g: 68, b: 204 } },
  { label: 'Cyberpunk Neon', color: { r: 0, g: 255, b: 255 } },
  { label: 'Crimson Red', color: { r: 255, g: 20, b: 40 } },
  { label: 'Emerald Green', color: { r: 0, g: 230, b: 100 } },
  { label: 'Electric Purple', color: { r: 160, g: 32, b: 240 } },
  { label: 'Amber Orange', color: { r: 255, g: 140, b: 0 } },
  { label: 'Pure White', color: { r: 255, g: 255, b: 255 } },
  { label: 'Stealth Off', color: { r: 0, g: 0, b: 0 } },
];

export const LightingStudio: React.FC<Props> = ({ lighting: initialLighting, onApply }) => {
  const { t } = useTranslation();
  const [lighting, setLighting] = useState<LightingConfig>(initialLighting);

  const getActiveColor = (): RgbColor => {
    if (lighting.lightbar.mode === 'static' || lighting.lightbar.mode === 'breathing') {
      return lighting.lightbar.color;
    }
    return { r: 0, g: 68, b: 204 };
  };

  const activeColor = getActiveColor();
  const hexString = `#${activeColor.r.toString(16).padStart(2, '0')}${activeColor.g.toString(16).padStart(2, '0')}${activeColor.b.toString(16).padStart(2, '0')}`;

  const setStaticColor = (color: RgbColor) => {
    const updated: LightingConfig = {
      ...lighting,
      lightbar: { mode: 'static', color },
    };
    setLighting(updated);
    onApply(updated);
  };

  return (
    <div className="lighting-studio-container">
      <div className="studio-header">
        <div>
          <h3>{t('dualsense_lighting_title')}</h3>
          <p className="section-subtitle">{t('dualsense_lighting_subtitle')}</p>
        </div>
      </div>

      <div className="lighting-grid">
        {/* Glow Controller Mockup */}
        <div className="deck-card lightbar-preview-card">
          <h4>💡 {t('dualsense_lightbar_preview')}</h4>
          <div className="controller-silhouette">
            <div
              className="touchpad-glow"
              style={{
                boxShadow: `0 0 25px 8px rgb(${activeColor.r}, ${activeColor.g}, ${activeColor.b})`,
                borderColor: `rgb(${activeColor.r}, ${activeColor.g}, ${activeColor.b})`,
              }}
            >
              <div className="touchpad-surface">
                <span>TOUCHPAD</span>
              </div>
            </div>
            {/* Player LED Bar */}
            <div className="player-led-bar">
              {['player4', 'all'].includes(lighting.player_led) && <span className="led-dot active" />}
              {['player2', 'player3', 'player4', 'all'].includes(lighting.player_led) && <span className="led-dot active" />}
              {['player1', 'player3', 'player4', 'all'].includes(lighting.player_led) && <span className="led-dot active" />}
              {['player2', 'player3', 'player4', 'all'].includes(lighting.player_led) && <span className="led-dot active" />}
              {['player4', 'all'].includes(lighting.player_led) && <span className="led-dot active" />}
            </div>
            {/* Mic LED */}
            <div className={`mic-led-indicator ${lighting.mic_led !== 'off' ? 'active' : ''}`}>
              <span>MIC</span>
            </div>
          </div>
        </div>

        {/* Color Palette & Mode Controls */}
        <div className="deck-card lighting-controls-card">
          <h4>🎨 {t('dualsense_lighting_palette')}</h4>

          <div className="form-group">
            <label>{t('dualsense_lighting_mode')}</label>
            <select
              className="deck-select"
              value={lighting.lightbar.mode}
              onChange={(e) => {
                const mode = e.target.value;
                let updated: LightingConfig = { ...lighting };
                if (mode === 'rainbow') {
                  updated = { ...lighting, lightbar: { mode: 'rainbow', speed: 1.0 } };
                } else if (mode === 'battery_level') {
                  updated = { ...lighting, lightbar: { mode: 'battery_level' } };
                } else if (mode === 'breathing') {
                  updated = { ...lighting, lightbar: { mode: 'breathing', color: activeColor, speed: 1.0 } };
                } else if (mode === 'off') {
                  updated = { ...lighting, lightbar: { mode: 'off' } };
                } else {
                  updated = { ...lighting, lightbar: { mode: 'static', color: activeColor } };
                }
                setLighting(updated);
                onApply(updated);
              }}
            >
              <option value="static">{t('dualsense_led_mode_static')}</option>
              <option value="breathing">{t('dualsense_led_mode_breathing')}</option>
              <option value="rainbow">{t('dualsense_led_mode_rainbow')}</option>
              <option value="battery_level">{t('dualsense_led_mode_battery')}</option>
              <option value="off">{t('dualsense_mode_off')}</option>
            </select>
          </div>

          <div className="color-swatches-grid">
            {COLOR_PRESETS.map((p) => (
              <button
                key={p.label}
                className="swatch-btn"
                style={{ backgroundColor: `rgb(${p.color.r}, ${p.color.g}, ${p.color.b})` }}
                onClick={() => setStaticColor(p.color)}
                title={p.label}
              />
            ))}
          </div>

          <div className="form-group hex-input-group">
            <label>{t('dualsense_custom_hex')}</label>
            <input
              type="color"
              className="color-picker-input"
              value={hexString}
              onChange={(e) => {
                const hex = e.target.value.replace('#', '');
                const r = parseInt(hex.substring(0, 2), 16);
                const g = parseInt(hex.substring(2, 4), 16);
                const b = parseInt(hex.substring(4, 6), 16);
                setStaticColor({ r, g, b });
              }}
            />
            <span className="hex-val-label">{hexString.toUpperCase()}</span>
          </div>

          {/* Player LEDs & Mic Mute */}
          <div className="form-group">
            <label>{t('dualsense_player_leds')}</label>
            <select
              className="deck-select"
              value={lighting.player_led}
              onChange={(e) => {
                const updated: LightingConfig = {
                  ...lighting,
                  player_led: e.target.value as PlayerLed,
                };
                setLighting(updated);
                onApply(updated);
              }}
            >
              <option value="player1">{t('dualsense_player_1')}</option>
              <option value="player2">{t('dualsense_player_2')}</option>
              <option value="player3">{t('dualsense_player_3')}</option>
              <option value="player4">{t('dualsense_player_4')}</option>
              <option value="all">{t('dualsense_player_all')}</option>
              <option value="off">{t('dualsense_mode_off')}</option>
            </select>
          </div>

          <div className="form-group">
            <label>{t('dualsense_mic_led')}</label>
            <select
              className="deck-select"
              value={lighting.mic_led}
              onChange={(e) => {
                const updated: LightingConfig = {
                  ...lighting,
                  mic_led: e.target.value as MicLed,
                };
                setLighting(updated);
                onApply(updated);
              }}
            >
              <option value="off">{t('dualsense_mic_off')}</option>
              <option value="solid">{t('dualsense_mic_solid')}</option>
              <option value="pulse">{t('dualsense_mic_pulse')}</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  );
};

