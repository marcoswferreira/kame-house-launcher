import React from 'react';
import { useTranslation } from '../../../i18n';
import { UdpServerStats } from '../../../types/models';

interface Props {
  stats: UdpServerStats;
  onToggle: (enabled: boolean) => void;
}

export const UdpServerStudio: React.FC<Props> = ({ stats, onToggle }) => {
  const { t } = useTranslation();

  return (
    <div className="udp-studio-container">
      <div className="studio-header">
        <div>
          <h3>{t('dualsense_udp_title')}</h3>
          <p className="section-subtitle">{t('dualsense_udp_subtitle')}</p>
        </div>
        <button
          className={`deck-btn ${stats.running ? 'danger' : 'primary'}`}
          onClick={() => onToggle(!stats.running)}
        >
          {stats.running ? t('dualsense_udp_stop') : t('dualsense_udp_start')}
        </button>
      </div>

      <div className="udp-metrics-grid">
        <div className="deck-card metric-card">
          <div className="metric-title">{t('dualsense_udp_status_label')}</div>
          <div className={`metric-value ${stats.running ? 'active-text' : 'disabled-text'}`}>
            {stats.running ? t('dualsense_udp_active') : t('dualsense_udp_stopped')}
          </div>
          <div className="metric-sub">UDP Port: {stats.port}</div>
        </div>

        <div className="deck-card metric-card">
          <div className="metric-title">{t('dualsense_udp_packets')}</div>
          <div className="metric-value">{stats.packets_received.toLocaleString()}</div>
          <div className="metric-sub">{t('dualsense_udp_packets_sub')}</div>
        </div>

        <div className="deck-card metric-card">
          <div className="metric-title">{t('dualsense_udp_last_client')}</div>
          <div className="metric-value client-ip-val">{stats.last_sender || '---'}</div>
          <div className="metric-sub">
            {stats.last_packet_time
              ? new Date(stats.last_packet_time).toLocaleTimeString()
              : t('dualsense_udp_no_traffic')}
          </div>
        </div>
      </div>

      <div className="deck-card udp-info-card">
        <h4>⚡ {t('dualsense_udp_compatible_title')}</h4>
        <p className="card-desc">{t('dualsense_udp_compatible_desc')}</p>
        <div className="mods-badge-list">
          <span className="badge-tag">Assetto Corsa</span>
          <span className="badge-tag">F1 23 / 24</span>
          <span className="badge-tag">Cyberpunk 2077</span>
          <span className="badge-tag">Spider-Man Remastered</span>
          <span className="badge-tag">Forza Horizon 5</span>
          <span className="badge-tag">DualSenseX Link V2</span>
        </div>
      </div>
    </div>
  );
};

