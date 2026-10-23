import { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { SunshineApp } from '../../../types/models';
import { Plus, RefreshCw, Trash2, Edit3 } from 'lucide-react';

interface StreamedAppsManagerProps {
  apps: SunshineApp[];
  onSyncAppsJson: () => void;
  onSaveApp: (app: SunshineApp) => void;
  onDeleteApp: (index: number) => void;
}

export function StreamedAppsManager({
  apps,
  onSyncAppsJson,
  onSaveApp,
  onDeleteApp,
}: StreamedAppsManagerProps) {
  const { t } = useTranslation();
  const [modalOpen, setModalOpen] = useState(false);
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [formName, setFormName] = useState('');
  const [formCmd, setFormCmd] = useState('');
  const [formWorkingDir, setFormWorkingDir] = useState('');
  const [formPrepDo, setFormPrepDo] = useState('');
  const [formPrepUndo, setFormPrepUndo] = useState('');

  const openNewAppModal = () => {
    setEditingIndex(null);
    setFormName('');
    setFormCmd('');
    setFormWorkingDir('');
    setFormPrepDo('');
    setFormPrepUndo('');
    setModalOpen(true);
  };

  const openEditAppModal = (app: SunshineApp, index: number) => {
    setEditingIndex(index);
    setFormName(app.name || '');
    setFormCmd(app.cmd || '');
    setFormWorkingDir(app.working_dir || '');
    setFormPrepDo(app.prep_cmd?.[0]?.do || '');
    setFormPrepUndo(app.prep_cmd?.[0]?.undo || '');
    setModalOpen(true);
  };

  const handleSave = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formName.trim()) return;

    const newApp: SunshineApp = {
      name: formName.trim(),
      cmd: formCmd.trim() || undefined,
      working_dir: formWorkingDir.trim() || undefined,
      auto_detach: 'true',
      wait_all: 'true',
      exit_timeout: '5',
      prep_cmd: formPrepDo || formPrepUndo ? [{ do: formPrepDo || undefined, undo: formPrepUndo || undefined, elevated: false }] : undefined,
    };

    onSaveApp(newApp);
    setModalOpen(false);
  };

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2 style={{ fontSize: '16px', fontWeight: 600, color: '#ffffff' }}>{t.apps_manager_title}</h2>
        <div style={{ display: 'flex', gap: '10px' }}>
          <button className="deck-btn deck-btn-primary focusable" onClick={openNewAppModal}>
            <Plus size={14} />
            <span>{t.btn_add_app}</span>
          </button>
          <button className="deck-btn focusable" onClick={onSyncAppsJson}>
            <RefreshCw size={14} />
            <span>{t.btn_sync_apps_json}</span>
          </button>
        </div>
      </div>

      <div className="table-container">
        <table className="deck-table">
          <thead>
            <tr>
              <th>{t.th_title}</th>
              <th>Command / Executable</th>
              <th>Working Directory</th>
              <th>Prep Commands</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {apps.length === 0 ? (
              <tr>
                <td colSpan={5} className="table-empty" style={{ textAlign: 'center', padding: '36px 16px', color: 'var(--deck-text-muted)' }}>
                  No apps configured in apps.json yet. Click "Sync Library to apps.json" or add a custom app.
                </td>
              </tr>
            ) : (
              apps.map((app, idx) => (
                <tr key={idx}>
                  <td style={{ color: '#ffffff', fontWeight: 600 }}>{app.name}</td>
                  <td className="mono-cell" title={app.cmd || ''}>{app.cmd || 'N/A'}</td>
                  <td className="mono-cell">{app.working_dir || 'Default'}</td>
                  <td>
                    {app.prep_cmd && app.prep_cmd.length > 0 ? (
                      <span className="platform-pill">{app.prep_cmd.length} script(s)</span>
                    ) : (
                      <span style={{ color: 'var(--deck-text-muted)' }}>None</span>
                    )}
                  </td>
                  <td>
                    <div style={{ display: 'flex', gap: '6px' }}>
                      <button
                        className="deck-btn focusable"
                        style={{ padding: '4px 8px', fontSize: '11px' }}
                        onClick={() => openEditAppModal(app, idx)}
                      >
                        <Edit3 size={11} />
                      </button>
                      <button
                        className="deck-btn deck-btn-danger focusable"
                        style={{ padding: '4px 8px', fontSize: '11px' }}
                        onClick={() => onDeleteApp(idx)}
                      >
                        <Trash2 size={11} />
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Add / Edit App Modal */}
      {modalOpen && (
        <div className="modal-overlay">
          <div className="modal-dialog">
            <div className="modal-header">
              {editingIndex !== null ? 'Edit Application' : t.modal_app_title}
            </div>
            <form onSubmit={handleSave} className="modal-form">
              <div className="form-group">
                <label className="form-label">{t.field_app_name}</label>
                <input
                  type="text"
                  className="deck-input focusable"
                  value={formName}
                  onChange={(e) => setFormName(e.target.value)}
                  placeholder="e.g. RetroArch / Cyberpunk 2077"
                  required
                />
              </div>

              <div className="form-group">
                <label className="form-label">{t.field_app_cmd}</label>
                <input
                  type="text"
                  className="deck-input focusable"
                  value={formCmd}
                  onChange={(e) => setFormCmd(e.target.value)}
                  placeholder="C:\Emulators\retroarch.exe"
                />
              </div>

              <div className="form-group">
                <label className="form-label">{t.field_app_working_dir}</label>
                <input
                  type="text"
                  className="deck-input focusable"
                  value={formWorkingDir}
                  onChange={(e) => setFormWorkingDir(e.target.value)}
                  placeholder="C:\Emulators"
                />
              </div>

              <div className="form-group">
                <label className="form-label">{t.field_app_prep_do}</label>
                <input
                  type="text"
                  className="deck-input focusable"
                  value={formPrepDo}
                  onChange={(e) => setFormPrepDo(e.target.value)}
                  placeholder="powershell.exe -File set_4k.ps1"
                />
              </div>

              <div className="form-group">
                <label className="form-label">{t.field_app_prep_undo}</label>
                <input
                  type="text"
                  className="deck-input focusable"
                  value={formPrepUndo}
                  onChange={(e) => setFormPrepUndo(e.target.value)}
                  placeholder="powershell.exe -File restore_res.ps1"
                />
              </div>

              <div className="modal-footer">
                <button type="button" className="deck-btn focusable" onClick={() => setModalOpen(false)}>
                  {t.btn_cancel}
                </button>
                <button type="submit" className="deck-btn deck-btn-primary focusable">
                  {t.btn_save_app}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

