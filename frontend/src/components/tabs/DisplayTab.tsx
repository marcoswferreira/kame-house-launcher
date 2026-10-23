import { useState } from 'react';
import { useTranslation } from '../../i18n';
import { StatusResponse } from '../../types/models';
import { FileCode } from 'lucide-react';

interface DisplayTabProps {
  status: StatusResponse | null;
  onUpdateLaunchBehavior: (val: string) => void;
  onDownloadAutoLoginReg: () => void;
}

export function DisplayTab({
  status,
  onUpdateLaunchBehavior,
  onDownloadAutoLoginReg,
}: DisplayTabProps) {
  const { t } = useTranslation();
  const [tvPreset, setTvPreset] = useState('4k120');
  const [hdrEnabled, setHdrEnabled] = useState(true);

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.display_title}</h1>
        <p className="panel-desc">{t.display_desc}</p>
      </div>

      <div className="settings-group">
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_launch_behavior}</div>
            <div className="setting-sub">{t.setting_launch_behavior_sub}</div>
          </div>
          <div className="setting-control">
            <select
              className="deck-select focusable"
              value={status?.launch_behavior || 'hide_and_restore_fullscreen'}
              onChange={(e) => onUpdateLaunchBehavior(e.target.value)}
              tabIndex={50}
            >
              <option value="hide_and_restore_fullscreen">{t.opt_hide_restore}</option>
              <option value="keep_open">{t.opt_keep_open}</option>
            </select>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_tv_resolution}</div>
            <div className="setting-sub">{t.setting_tv_resolution_sub}</div>
          </div>
          <div className="setting-control">
            <select
              className="deck-select focusable"
              value={tvPreset}
              onChange={(e) => setTvPreset(e.target.value)}
              tabIndex={51}
            >
              <option value="4k120">3840 x 2160 @ 120Hz (Recommended for 4K OLED)</option>
              <option value="1440p120">2560 x 1440 @ 120Hz (High Performance)</option>
              <option value="1080p120">1920 x 1080 @ 120Hz (FHD 120fps)</option>
              <option value="1080p60">1920 x 1080 @ 60Hz (Standard FHD)</option>
            </select>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_hdr}</div>
            <div className="setting-sub">{t.setting_hdr_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={hdrEnabled}
                onChange={(e) => setHdrEnabled(e.target.checked)}
                className="focusable"
                tabIndex={52}
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_autologin}</div>
            <div className="setting-sub">{t.setting_autologin_sub}</div>
          </div>
          <div className="setting-control">
            <button className="deck-btn focusable" onClick={onDownloadAutoLoginReg} tabIndex={53}>
              <FileCode size={15} />
              <span>{t.btn_generate_reg}</span>
            </button>
          </div>
        </div>
      </div>
    </section>
  );
}

