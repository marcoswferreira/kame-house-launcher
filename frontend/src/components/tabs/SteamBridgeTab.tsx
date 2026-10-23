import { useState } from 'react';
import { useTranslation } from '../../i18n';
import { Game } from '../../types/models';
import { RefreshCw, FolderSearch, FolderCog, Play, Plus, Trash2, X } from 'lucide-react';

interface SteamBridgeTabProps {
  games: Game[];
  scanDirectories: string[];
  onSyncSteam: () => void;
  onScanGames: (paths?: string[]) => void;
  onUpdateScanDirectories: (paths: string[]) => void;
  onLaunchGame: (id: string) => void;
}

export function SteamBridgeTab({
  games,
  scanDirectories,
  onSyncSteam,
  onScanGames,
  onUpdateScanDirectories,
  onLaunchGame,
}: SteamBridgeTabProps) {
  const { t } = useTranslation();
  const [cdpEnabled, setCdpEnabled] = useState(true);
  const [tagsEnabled, setTagsEnabled] = useState(true);
  const [modalOpen, setModalOpen] = useState(false);
  const [newPath, setNewPath] = useState('');

  const truncate = (str: string, len: number) => {
    if (!str || str.length <= len) return str;
    const parts = str.split(/[/\\]/);
    const filename = parts.pop();
    return `.../${filename}`;
  };

  const handleAddDirectory = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = newPath.trim();
    if (!trimmed || scanDirectories.includes(trimmed)) return;
    const updated = [...scanDirectories, trimmed];
    onUpdateScanDirectories(updated);
    setNewPath('');
  };

  const handleRemoveDirectory = (index: number) => {
    const updated = scanDirectories.filter((_, i) => i !== index);
    onUpdateScanDirectories(updated);
  };

  const handleScanFromModal = () => {
    onScanGames(scanDirectories);
    setModalOpen(false);
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
        <button className="deck-btn focusable" onClick={() => onScanGames()} tabIndex={21} title="Scan all saved directories">
          <FolderSearch size={15} />
          <span>{t.btn_scan_folders}</span>
        </button>
        <button className="deck-btn focusable" onClick={() => setModalOpen(true)} tabIndex={22} title="Choose custom folders">
          <FolderCog size={15} />
          <span>{t.btn_manage_folders}</span>
        </button>
      </div>

      <div className="settings-group">
        <div className="setting-row">
          <div className="setting-info">
            <div className="setting-label">{t.setting_deterministic_appids}</div>
            <div className="setting-sub">{t.setting_deterministic_appids_sub}</div>
          </div>
          <div className="setting-control">
            <span
              className="status-badge active"
              style={{
                display: 'inline-block',
                padding: '3px 8px',
                borderRadius: '3px',
                fontSize: '10px',
                fontWeight: 700,
                backgroundColor: 'rgba(92, 191, 71, 0.15)',
                border: '1px solid rgba(92, 191, 71, 0.4)',
                color: 'var(--deck-green)',
              }}
            >
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
                tabIndex={23}
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
                tabIndex={24}
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
                <td
                  colSpan={5}
                  className="table-empty"
                  style={{ textAlign: 'center', padding: '36px 16px', color: 'var(--deck-text-muted)' }}
                >
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

      {/* Manage Scan Folders Modal */}
      {modalOpen && (
        <div className="modal-overlay">
          <div className="modal-dialog" style={{ width: '600px' }}>
            <div className="modal-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <span>{t.modal_folders_title}</span>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p style={{ fontSize: '12px', color: 'var(--deck-text-secondary)', marginBottom: '16px' }}>
              {t.modal_folders_desc}
            </p>

            {/* List of active folders */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginBottom: '18px', maxHeight: '180px', overflowY: 'auto' }}>
              {scanDirectories.length === 0 ? (
                <div style={{ fontSize: '12px', color: 'var(--deck-text-muted)', padding: '12px', backgroundColor: 'var(--deck-card-bg)', borderRadius: '4px' }}>
                  {t.no_folders_configured}
                </div>
              ) : (
                scanDirectories.map((dir, idx) => (
                  <div
                    key={idx}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      backgroundColor: 'var(--deck-card-bg)',
                      border: '1px solid var(--deck-border)',
                      padding: '8px 12px',
                      borderRadius: '4px',
                    }}
                  >
                    <span className="mono-cell" style={{ color: 'var(--deck-text-primary)' }}>{dir}</span>
                    <button
                      className="deck-btn deck-btn-danger focusable"
                      style={{ padding: '4px 8px', fontSize: '11px' }}
                      onClick={() => handleRemoveDirectory(idx)}
                    >
                      <Trash2 size={12} />
                    </button>
                  </div>
                ))
              )}
            </div>

            {/* Add new folder form */}
            <form onSubmit={handleAddDirectory} style={{ display: 'flex', gap: '8px', marginBottom: '20px' }}>
              <input
                type="text"
                className="deck-input focusable"
                style={{ flex: 1 }}
                value={newPath}
                onChange={(e) => setNewPath(e.target.value)}
                placeholder={t.field_add_folder}
              />
              <button type="submit" className="deck-btn focusable">
                <Plus size={14} />
                <span>{t.btn_add_folder}</span>
              </button>
            </form>

            <div className="modal-footer" style={{ display: 'flex', justifyContent: 'space-between' }}>
              <button type="button" className="deck-btn focusable" onClick={() => setModalOpen(false)}>
                {t.btn_cancel}
              </button>
              <button
                type="button"
                className="deck-btn deck-btn-primary focusable"
                onClick={handleScanFromModal}
              >
                <FolderSearch size={14} />
                <span>{t.btn_scan_all_folders}</span>
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
