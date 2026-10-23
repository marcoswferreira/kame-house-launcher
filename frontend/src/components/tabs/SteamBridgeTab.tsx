import { useState } from 'react';
import { useTranslation } from '../../i18n';
import { Game } from '../../types/models';
import {
  RefreshCw,
  FolderSearch,
  FolderCog,
  Plus,
  Trash2,
  X,
  Compass,
  CheckCircle2,
  AlertTriangle,
  RotateCcw,
} from 'lucide-react';

interface SteamBridgeTabProps {
  games: Game[];
  scanDirectories: string[];
  steamShortcutsPath?: string;
  steamVdfIsReal?: boolean;
  steamVdfIsCustom?: boolean;
  onSyncSteam: () => void;
  onScanGames: (paths?: string[]) => void;
  onUpdateScanDirectories: (paths: string[]) => void;
  onRemoveGame?: (id: string, title: string) => void;
  onConfigureSteamPath?: (path: string | null) => Promise<boolean>;
}

export function SteamBridgeTab({
  games,
  scanDirectories,
  steamShortcutsPath,
  steamVdfIsReal,
  steamVdfIsCustom,
  onSyncSteam,
  onScanGames,
  onUpdateScanDirectories,
  onRemoveGame,
  onConfigureSteamPath,
}: SteamBridgeTabProps) {
  const { t, lang } = useTranslation();
  const [cdpEnabled, setCdpEnabled] = useState(true);
  const [tagsEnabled, setTagsEnabled] = useState(true);
  const [modalOpen, setModalOpen] = useState(false);
  const [newPath, setNewPath] = useState('');

  // Steam Location modal state
  const [steamModalOpen, setSteamModalOpen] = useState(false);
  const [steamInputPath, setSteamInputPath] = useState('');
  const [candidates, setCandidates] = useState<string[]>([]);
  const [loadingCandidates, setLoadingCandidates] = useState(false);

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

  const handleOpenSteamModal = async () => {
    setSteamInputPath(steamShortcutsPath || '');
    setSteamModalOpen(true);
    setLoadingCandidates(true);
    try {
      const res = await fetch('/api/steam/path');
      if (res.ok) {
        const data = await res.json();
        setCandidates(data.candidates || []);
        if (data.path) {
          setSteamInputPath(data.path);
        }
      }
    } catch {
      // ignore
    } finally {
      setLoadingCandidates(false);
    }
  };

  const handleSaveSteamPath = async (e: React.FormEvent) => {
    e.preventDefault();
    const ok = await onConfigureSteamPath?.(steamInputPath.trim() || null);
    if (ok) {
      setSteamModalOpen(false);
    }
  };

  const handleResetSteamPath = async () => {
    const ok = await onConfigureSteamPath?.(null);
    if (ok) {
      setSteamModalOpen(false);
    }
  };

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.steam_bridge_title}</h1>
        <p className="panel-desc">{t.steam_bridge_desc}</p>
      </div>

      {/* Target Steam Shortcuts Destination Card */}
      <div
        className="deck-card"
        style={{
          marginBottom: '20px',
          padding: '14px 18px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          gap: '16px',
          flexWrap: 'wrap',
          borderLeft: steamVdfIsCustom
            ? '4px solid #3b82f6'
            : steamVdfIsReal
              ? '4px solid var(--deck-green)'
              : '4px solid #f59e0b',
        }}
      >
        <div style={{ display: 'flex', flexDirection: 'column', gap: '4px', minWidth: '260px', flex: 1 }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span style={{ fontSize: '11px', fontWeight: 700, color: '#e5e7eb', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
              {t.current_steam_vdf}
            </span>
            <span
              className="status-badge active"
              style={{
                padding: '2px 8px',
                borderRadius: '3px',
                fontSize: '10px',
                fontWeight: 700,
                backgroundColor: steamVdfIsCustom
                  ? 'rgba(59, 130, 246, 0.15)'
                  : steamVdfIsReal
                    ? 'rgba(92, 191, 71, 0.15)'
                    : 'rgba(245, 158, 11, 0.15)',
                border: `1px solid ${
                  steamVdfIsCustom
                    ? 'rgba(59, 130, 246, 0.4)'
                    : steamVdfIsReal
                      ? 'rgba(92, 191, 71, 0.4)'
                      : 'rgba(245, 158, 11, 0.4)'
                }`,
                color: steamVdfIsCustom
                  ? '#60a5fa'
                  : steamVdfIsReal
                    ? 'var(--deck-green)'
                    : '#f59e0b',
              }}
            >
              {steamVdfIsCustom
                ? t.badge_steam_custom
                : steamVdfIsReal
                  ? t.badge_steam_detected
                  : t.badge_steam_not_found}
            </span>
          </div>
          <div
            className="mono-cell"
            style={{
              fontSize: '12px',
              color: steamVdfIsReal || steamVdfIsCustom ? 'var(--deck-text-secondary)' : '#f87171',
              wordBreak: 'break-all',
            }}
            title={steamShortcutsPath}
          >
            {steamShortcutsPath || (lang === 'pt' ? 'Nenhum caminho detectado' : 'No path detected')}
          </div>
        </div>

        <button
          className="deck-btn focusable"
          onClick={handleOpenSteamModal}
          tabIndex={19}
          style={{ display: 'inline-flex', alignItems: 'center', gap: '6px', padding: '8px 14px' }}
        >
          <Compass size={14} />
          <span>{t.btn_configure_steam_path}</span>
        </button>
      </div>

      <div className="bridge-action-bar" style={{ display: 'flex', gap: '12px', marginBottom: '20px', flexWrap: 'wrap' }}>
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
              <th>{t.th_status}</th>
              <th style={{ width: '130px', textAlign: 'center' }}>{t.th_actions}</th>
            </tr>
          </thead>
          <tbody>
            {games.length === 0 ? (
              <tr>
                <td
                  colSpan={6}
                  className="table-empty"
                  style={{ textAlign: 'center', padding: '36px 16px', color: 'var(--deck-text-muted)' }}
                >
                  {t.table_empty}
                </td>
              </tr>
            ) : (
              games.map((game, idx) => {
                const gameKey = game.id || (game as { uuid?: string }).uuid || game.path || String(idx);
                return (
                  <tr key={gameKey}>
                    <td style={{ color: '#ffffff', fontWeight: 600 }}>{game.title}</td>
                    <td>
                      <span className="platform-pill">
                        {game.platform_name || game.platform || 'PC'}
                      </span>
                    </td>
                    <td className="mono-cell" title={game.custom_exe || game.path}>
                      {truncate(game.custom_exe || game.path, 34)}
                    </td>
                    <td className="mono-cell">{game.shortcut_id || 'AUTO-CRC'}</td>
                    <td>
                      <span
                        className="status-badge active"
                        style={{
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                          padding: '4px 10px',
                          borderRadius: '4px',
                          fontSize: '11px',
                          fontWeight: 600,
                          backgroundColor: 'rgba(92, 191, 71, 0.15)',
                          border: '1px solid rgba(92, 191, 71, 0.35)',
                          color: 'var(--deck-green)',
                        }}
                      >
                        <span
                          style={{
                            width: '6px',
                            height: '6px',
                            borderRadius: '50%',
                            backgroundColor: 'var(--deck-green)',
                          }}
                        />
                        <span>{t.status_steam_ready}</span>
                      </span>
                    </td>
                    <td style={{ textAlign: 'center' }}>
                      <button
                        className="deck-btn deck-btn-danger focusable"
                        style={{
                          padding: '5px 10px',
                          fontSize: '11px',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                          borderColor: 'rgba(239, 68, 68, 0.4)',
                          color: '#f87171',
                        }}
                        onClick={() => onRemoveGame?.(gameKey, game.title)}
                        title={t.btn_remove_game}
                      >
                        <Trash2 size={13} />
                        <span>{t.btn_remove_game}</span>
                      </button>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      {/* Point Steam Location Modal */}
      {steamModalOpen && (
        <div className="modal-overlay">
          <div className="modal-dialog" style={{ width: '640px' }}>
            <div className="modal-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Compass size={18} style={{ color: '#3b82f6' }} />
                <span>{t.modal_steam_path_title}</span>
              </div>
              <button
                className="deck-btn focusable"
                style={{ padding: '4px', background: 'transparent', border: 'none' }}
                onClick={() => setSteamModalOpen(false)}
              >
                <X size={16} />
              </button>
            </div>

            <p style={{ fontSize: '13px', color: 'var(--deck-text-secondary)', lineHeight: 1.5, marginBottom: '16px' }}>
              {t.modal_steam_path_desc}
            </p>

            {/* List of Detected Steam candidate paths if available */}
            {candidates.length > 0 && (
              <div style={{ marginBottom: '16px' }}>
                <div style={{ fontSize: '12px', fontWeight: 600, color: '#e5e7eb', marginBottom: '8px' }}>
                  {t.steam_detected_candidates}
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', maxHeight: '140px', overflowY: 'auto' }}>
                  {candidates.map((cand, i) => (
                    <button
                      key={i}
                      type="button"
                      className="deck-btn focusable"
                      style={{
                        textAlign: 'left',
                        justifyContent: 'flex-start',
                        padding: '8px 12px',
                        fontSize: '11.5px',
                        borderColor: steamInputPath === cand ? 'var(--deck-blue)' : 'var(--deck-border)',
                        backgroundColor: steamInputPath === cand ? 'rgba(59, 130, 246, 0.15)' : 'var(--deck-card-bg)',
                      }}
                      onClick={() => setSteamInputPath(cand)}
                    >
                      <CheckCircle2 size={13} style={{ color: steamInputPath === cand ? '#60a5fa' : 'var(--deck-text-muted)', flexShrink: 0 }} />
                      <span className="mono-cell" style={{ wordBreak: 'break-all' }}>{cand}</span>
                    </button>
                  ))}
                </div>
              </div>
            )}

            {/* Steam Path Form */}
            <form onSubmit={handleSaveSteamPath}>
              <div style={{ marginBottom: '16px' }}>
                <label style={{ display: 'block', fontSize: '12px', fontWeight: 600, color: '#e5e7eb', marginBottom: '6px' }}>
                  {t.steam_path_label}
                </label>
                <input
                  type="text"
                  className="deck-input focusable"
                  style={{ width: '100%', fontFamily: 'var(--font-mono)', fontSize: '12.5px' }}
                  value={steamInputPath}
                  onChange={(e) => setSteamInputPath(e.target.value)}
                  placeholder={t.steam_path_placeholder}
                />
              </div>

              <div className="modal-footer" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '20px' }}>
                <button
                  type="button"
                  className="deck-btn focusable"
                  style={{ display: 'inline-flex', alignItems: 'center', gap: '6px' }}
                  onClick={handleResetSteamPath}
                  title={t.btn_reset_steam_path}
                >
                  <RotateCcw size={13} />
                  <span>{t.btn_reset_steam_path}</span>
                </button>

                <div style={{ display: 'flex', gap: '8px' }}>
                  <button type="button" className="deck-btn focusable" onClick={() => setSteamModalOpen(false)}>
                    {t.btn_cancel}
                  </button>
                  <button type="submit" className="deck-btn deck-btn-primary focusable">
                    {t.btn_save_steam_path}
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
      )}

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
