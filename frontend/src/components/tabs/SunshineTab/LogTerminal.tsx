import { useState, useEffect } from 'react';
import { useTranslation } from '../../../i18n';
import { RefreshCw, RotateCcw, Trash2, Monitor } from 'lucide-react';

interface LogTerminalProps {
  logs: string;
  onRefreshLogs: () => void;
  onRestartSunshine: () => void;
  onResetDisplay: () => void;
}

export function LogTerminal({
  logs,
  onRefreshLogs,
  onRestartSunshine,
  onResetDisplay,
}: LogTerminalProps) {
  const { t } = useTranslation();
  const [displayedLogs, setDisplayedLogs] = useState(logs);

  useEffect(() => {
    setDisplayedLogs(logs);
  }, [logs]);

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
        <h2 style={{ fontSize: '16px', fontWeight: 600, color: '#ffffff' }}>{t.logs_title}</h2>
        <div style={{ display: 'flex', gap: '8px' }}>
          <button className="deck-btn focusable" style={{ padding: '6px 12px', fontSize: '11px' }} onClick={onRefreshLogs}>
            <RefreshCw size={12} />
            <span>{t.btn_refresh_logs}</span>
          </button>
          <button
            className="deck-btn focusable"
            style={{ padding: '6px 12px', fontSize: '11px' }}
            onClick={() => setDisplayedLogs('--- Logs Cleared ---')}
          >
            <Trash2 size={12} />
            <span>{t.btn_clear_logs}</span>
          </button>
          <button
            className="deck-btn deck-btn-primary focusable"
            style={{ padding: '6px 12px', fontSize: '11px' }}
            onClick={onRestartSunshine}
          >
            <RotateCcw size={12} />
            <span>{t.btn_restart_sunshine}</span>
          </button>
          <button
            className="deck-btn focusable"
            style={{ padding: '6px 12px', fontSize: '11px' }}
            onClick={onResetDisplay}
            title="Reset virtual display device persistence"
          >
            <Monitor size={12} />
            <span>Reset Display</span>
          </button>
        </div>
      </div>

      <div className="log-terminal">
        {displayedLogs || 'Connecting to Sunshine daemon logger at https://localhost:47990/api/logs...'}
      </div>
    </div>
  );
}

