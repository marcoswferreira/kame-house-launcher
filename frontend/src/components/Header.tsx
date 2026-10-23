import { useEffect, useState } from 'react';
import { useTranslation } from '../i18n';
import { StatusResponse } from '../types/models';
import { Gamepad2, Globe, Terminal, X } from 'lucide-react';

interface HeaderProps {
  status: StatusResponse | null;
  isGamepadActive: boolean;
  onToggleDebugConsole?: () => void;
  onRequestClose?: () => void;
}

export function Header({
  status,
  isGamepadActive,
  onToggleDebugConsole,
  onRequestClose,
}: HeaderProps) {
  const { lang, toggleLang, t } = useTranslation();
  const [time, setTime] = useState('');

  useEffect(() => {
    const updateTime = () => {
      setTime(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }));
    };
    updateTime();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <header className="deck-header">
      <div className="header-branding">
        <img src="/logo.svg" alt="Kame House" className="brand-logo" />
        <div className="brand-meta">
          <span className="brand-title">KAME HOUSE</span>
          <span className="brand-subtitle">{t.app_subtitle}</span>
        </div>
      </div>

      <div className="header-hud">
        {/* DualSense status chip */}
        <div className="hud-chip" title="DualSense Controller & UDP Link">
          <span className={`chip-dot ${status?.dualsense_connected ? '' : 'off'}`} />
          <span className="chip-label">DUALSENSE</span>
          <span className="chip-val">
            {status?.dualsense_connected
              ? status?.dualsense_udp_enabled
                ? ':6969'
                : 'READY'
              : 'STANDBY'}
          </span>
        </div>

        {/* Steam VDF status chip */}
        <div className="hud-chip status-ok" title="Steam shortcuts.vdf Bridge">
          <span className="chip-dot" />
          <span className="chip-label">STEAM VDF</span>
          <span className="chip-val">{t.hud_ready}</span>
        </div>

        {/* Gamepad chip */}
        <div className="hud-chip" title="Gamepad Input Status">
          <Gamepad2 size={13} style={{ color: isGamepadActive ? 'var(--deck-green)' : 'var(--deck-text-muted)' }} />
          <span className="chip-label">{isGamepadActive ? 'CONNECTED' : 'KEYBOARD'}</span>
        </div>

        {/* Toggle Debug Terminal button */}
        {onToggleDebugConsole && (
          <button
            className="hud-pill-btn focusable"
            onClick={onToggleDebugConsole}
            title={
              lang === 'pt'
                ? 'Mostrar/Ocultar Terminal de Debug (também disponível no System Tray)'
                : 'Show/Hide Debug Terminal (also available in System Tray)'
            }
            style={{
              borderColor: status?.debug_console_visible ? 'var(--deck-blue)' : undefined,
              color: status?.debug_console_visible ? 'var(--deck-blue)' : undefined,
            }}
          >
            <Terminal size={13} />
            <span>{lang === 'pt' ? 'DEBUG' : 'DEBUG'}</span>
          </button>
        )}

        {/* Language switch button */}
        <button className="hud-pill-btn focusable" onClick={toggleLang} title="Switch Language (Y)" tabIndex={1}>
          <Globe size={13} />
          <span>{lang === 'pt' ? 'PT-BR' : 'EN'}</span>
        </button>

        {/* System Clock */}
        <div className="deck-clock">{time}</div>

        {/* Close / Minimize to System Tray button */}
        {onRequestClose && (
          <button
            className="hud-pill-btn focusable"
            onClick={onRequestClose}
            title={
              lang === 'pt'
                ? 'Fechar ou Minimizar para a Bandeja (System Tray)'
                : 'Close or Minimize to System Tray'
            }
            style={{
              padding: '5px 8px',
              borderColor: 'rgba(239, 68, 68, 0.35)',
              color: '#f87171',
            }}
          >
            <X size={14} />
          </button>
        )}
      </div>
    </header>
  );
}

