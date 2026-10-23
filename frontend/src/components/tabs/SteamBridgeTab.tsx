import { useState } from 'react';
import { useTranslation } from '../../i18n';
import { Game } from '../../types/models';
import { RefreshCw, FolderSearch, Play } from 'lucide-react';

interface SteamBridgeTabProps {
  games: Game[];
  onSyncSteam: () => void;
  onScanGames: () => void;
  onLaunchGame: (id: string) => void;
}

export function SteamBridgeTab({
  games,
  onSyncSteam,
  onScanGames,
  onLaunchGame,
}: SteamBridgeTabProps) {
  const { t } = useTranslation();
  const [cdpEnabled, setCdpEnabled] = useState(true);
  const [tagsEnabled, setTagsEnabled] = useState(true);

  const truncate = (str: string, len: number) => {
    if (!str || str.length <= len) return str;
    const parts = str.split(/[/\\]/);
    const filename = parts.pop();
    return `.../${filename}`;
  };

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.steam_bridge_title}</h1>
        <p className="panel-desc">{t.steam_bridge_desc}</p>
      </div>

      <div className="bridge-action-bar" style={{ display: 'flex', gap: '12px', marginBottom: '20px' }}>
        <button className="deck-btn deck-btn-primary focusable" onClick={onSyncSteam} tabIndex={20}>
          <RefreshCw size={15} />
          <span>{t.btn_sync_shortcuts_now}</span>
        </button>
        <button className="deck-btn focusable" onClick={onScanGames} tabIndex={21}>
          <FolderSearch size={15} />
          <span>{t.btn_scan_folders}</span>
        </button>
      </div>

      <div className="settings-group">
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_deterministic_appids}</div>
            <div className="setting-sub">{t.setting_deterministic_appids_sub}</div>
          </div>
          <div className="setting-control">
            <span className="status-badge active" style={{
              display: 'inline-block',
              padding: '3px 8px',
              borderRadius: '3px',
              fontSize: '10px',
              fontWeight: 700,
              backgroundColor: 'rgba(92, 191, 71, 0.15)',
              border: '1px solid rgba(92, 191, 71, 0.4)',
              color: 'var(--deck-green)'
            }}>
              {t.badge_enforced}
            </span>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_cdp_listener}</div>
            <div className="setting-sub">{t.setting_cdp_listener_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={cdpEnabled}
                onChange={(e) => setCdpEnabled(e.target.checked)}
                className="focusable"
                tabIndex={22}
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>

        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_steam_tags}</div>
            <div className="setting-sub">{t.setting_steam_tags_sub}</div>
          </div>
          <div className="setting-control">
            <label className="deck-switch">
              <input
                type="checkbox"
                checked={tagsEnabled}
                onChange={(e) => setTagsEnabled(e.target.checked)}
                className="focusable"
                tabIndex={23}
              />
              <span className="switch-slider" />
            </label>
          </div>
        </div>
      </div>

      <div className="section-divider">
        <span className="divider-title">{t.discovered_games_title}</span>
        <span className="divider-meta">{games.length} items ready</span>
      </div>

      <div className="table-container">
        <table className="deck-table">
          <thead>
            <tr>
              <th>{t.th_title}</th>
              <th>{t.th_platform}</th>
              <th>{t.th_path}</th>
              <th>{t.th_appid}</th>
              <th>{t.th_action}</th>
            </tr>
          </thead>
          <tbody>
            {games.length === 0 ? (
              <tr>
                <td colSpan={5} className="table-empty" style={{ textAlign: 'center', padding: '36px 16px', color: 'var(--deck-text-muted)' }}>
                  {t.table_empty}
                </td>
              </tr>
            ) : (
              games.map((game, idx) => (
                <tr key={game.id || idx}>
                  <td style={{ color: '#ffffff', fontWeight: 600 }}>{game.title}</td>
                  <td>
                    <span className="platform-pill">
                      {game.platform_name || game.platform || 'PC'}
                    </span>
                  </td>
                  <td className="mono-cell" title={game.custom_exe || game.path}>
                    {truncate(game.custom_exe || game.path, 36)}
                  </td>
                  <td className="mono-cell">{game.shortcut_id || 'AUTO-CRC'}</td>
                  <td>
                    <button
                      className="deck-btn deck-btn-primary focusable"
                      style={{ padding: '5px 12px', fontSize: '11px' }}
                      onClick={() => onLaunchGame(game.id)}
                      tabIndex={100 + idx}
                    >
                      <Play size={11} />
                      <span>{t.btn_launch}</span>
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}

