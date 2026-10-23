import { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { SunshineConfig } from '../../../types/models';
import { Cpu, Sliders, Volume2, Save } from 'lucide-react';

interface AVSettingsProps {
  config: SunshineConfig;
  onSaveConfig: (patch: Partial<SunshineConfig>) => void;
}

export function AVSettings({ config, onSaveConfig }: AVSettingsProps) {
  const { t } = useTranslation();
  const [encoder, setEncoder] = useState(config.encoder || 'auto');
  const [maxBitrate, setMaxBitrate] = useState(config.max_bitrate || 0);
  const [fpsTarget, setFpsTarget] = useState(config.minimum_fps_target || 60);
  const [nvPreset, setNvPreset] = useState(config.nvenc_preset || 1);
  const [amdQuality, setAmdQuality] = useState(config.amd_quality || 'balanced');
  const [qsvPreset, setQsvPreset] = useState(config.qsv_preset || 'medium');
  const [streamAudio, setStreamAudio] = useState(config.stream_audio !== 'disabled');

  const handleSave = () => {
    onSaveConfig({
      encoder: encoder === 'auto' ? '' : encoder,
      max_bitrate: maxBitrate,
      minimum_fps_target: fpsTarget,
      nvenc_preset: nvPreset,
      amd_quality: amdQuality,
      qsv_preset: qsvPreset,
      stream_audio: streamAudio ? 'enabled' : 'disabled',
    });
  };

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2 style={{ fontSize: '16px', fontWeight: 600, color: '#ffffff' }}>{t.av_settings_title}</h2>
        <button className="deck-btn deck-btn-primary focusable" onClick={handleSave}>
          <Save size={14} />
          <span>Save Changes</span>
        </button>
      </div>

      <div className="settings-group">
        {/* Encoder Selector */}
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_encoder_choice}</div>
            <div className="setting-sub">{t.setting_encoder_choice_sub}</div>
          </div>
          <div className="setting-control">
            <select
              className="deck-select focusable"
              value={encoder}
              onChange={(e) => setEncoder(e.target.value)}
            >
              <option value="auto">Auto-Detect GPU (Recommended)</option>
              <option value="nvenc">NVIDIA NVENC (GeForce RTX / GTX)</option>
              <option value="amf">AMD AMF (Radeon RX)</option>
              <option value="qsv">Intel QuickSync (Arc / Iris Xe)</option>
              <option value="software">Software (CPU x264/x265)</option>
            </select>
          </div>
        </div>

        {/* NVIDIA NVENC Details */}
        {encoder === 'nvenc' && (
          <div className="setting-row">
            <div className="setting-info">
              <div className="setting-label">NVENC Performance Preset</div>
              <div className="setting-sub">P1 (Fastest / Lowest Latency) to P7 (Highest Quality).</div>
            </div>
            <div className="setting-control">
              <select
                className="deck-select focusable"
                value={nvPreset}
                onChange={(e) => setNvPreset(parseInt(e.target.value, 10))}
              >
                <option value={1}>P1: Lowest Latency (Console Standard)</option>
                <option value={2}>P2: Fast</option>
                <option value={4}>P4: Balanced</option>
                <option value={7}>P7: Max Quality</option>
              </select>
            </div>
          </div>
        )}

        {/* AMD AMF Details */}
        {encoder === 'amf' && (
          <div className="setting-row">
            <div className="setting-info">
              <div className="setting-label">AMD AMF Quality Profile</div>
              <div className="setting-sub">Tuning mode for AMD Radeon video encoding engine.</div>
            </div>
            <div className="setting-control">
              <select
                className="deck-select focusable"
                value={amdQuality}
                onChange={(e) => setAmdQuality(e.target.value)}
              >
                <option value="speed">Speed (Lowest Latency)</option>
                <option value="balanced">Balanced</option>
                <option value="quality">Quality</option>
              </select>
            </div>
          </div>
        )}

        {/* Intel QuickSync Details */}
        {encoder === 'qsv' && (
          <div className="setting-row">
            <div className="setting-info">
              <div className="setting-label">Intel QSV Tuning</div>
              <div className="setting-sub">Intel QuickSync video encoder preset.</div>
            </div>
            <div className="setting-control">
              <select
                className="deck-select focusable"
                value={qsvPreset}
                onChange={(e) => setQsvPreset(e.target.value)}
              >
                <option value="veryfast">Very Fast</option>
                <option value="faster">Faster</option>
                <option value="medium">Medium</option>
              </select>
            </div>
          </div>
        )}

        {/* Bitrate */}
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_max_bitrate}</div>
            <div className="setting-sub">
              {maxBitrate === 0 ? 'Automatic (Requested by Moonlight Client)' : `${maxBitrate} Mbps`}
            </div>
          </div>
          <div className="setting-control" style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <input
              type="range"
              min={0}
              max={150}
              step={5}
              value={maxBitrate}
              onChange={(e) => setMaxBitrate(parseInt(e.target.value, 10))}
              style={{ width: '180px', accentColor: 'var(--deck-blue)' }}
            />
            <span className="mono-cell" style={{ width: '60px' }}>
              {maxBitrate === 0 ? 'AUTO' : `${maxBitrate}M`}
            </span>
          </div>
        </div>

        {/* FPS Target */}
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_target_fps}</div>
            <div className="setting-sub">{t.setting_target_fps_sub}</div>
          </div>
          <div className="setting-control">
            <select
              className="deck-select focusable"
              value={fpsTarget}
              onChange={(e) => setFpsTarget(parseInt(e.target.value, 10))}
            >
              <option value={60}>60 FPS (Living Room Standard)</option>
              <option value={90}>90 FPS (Steam Deck OLED)</option>
              <option value={120}>120 FPS (High Refresh TV)</option>
              <option value={0}>Unlimited / Client Dictated</option>
            </select>
          </div>
        </div>

        {/* Audio Sink */}
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_audio_sink}</div>
            <div className="setting-sub">{t.setting_audio_sink_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={streamAudio}
                onChange={(e) => setStreamAudio(e.target.checked)}
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

