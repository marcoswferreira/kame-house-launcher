import { useTranslation } from '../../i18n';
import { Moon, RefreshCw, Power } from 'lucide-react';

interface PowerTabProps {
  onPowerAction: (action: 'sleep' | 'restart' | 'shutdown') => void;
  gamepadTelemetry: {
    name: string;
    connected: boolean;
    leftStick: { x: number; y: number };
    rightStick: { x: number; y: number };
  };
}

export function PowerTab({ onPowerAction, gamepadTelemetry }: PowerTabProps) {
  const { t } = useTranslation();

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.power_title}</h1>
        <p className="panel-desc">{t.power_desc}</p>
      </div>

      <div className="power-grid" style={{ display: 'flex', flexDirection: 'column', gap: '10px', marginBottom: '24px' }}>
        <div
          className="power-card focusable"
          style={{ display: 'flex', alignItems: 'center', gap: '20px', padding: '18px 24px' }}
          onClick={() => onPowerAction('sleep')}
          tabIndex={60}
          role="button"
        >
          <div className="power-icon" style={{ fontSize: '24px', color: 'var(--deck-focus)' }}>
            <Moon size={24} />
          </div>
          <div className="power-info" style={{ flex: 1 }}>
            <div className="power-action-title">{t.power_sleep_title}</div>
            <div className="power-action-desc">{t.power_sleep_desc}</div>
          </div>
          <button className="deck-btn deck-btn-primary">{t.btn_sleep}</button>
        </div>

        <div
          className="power-card focusable"
          style={{ display: 'flex', alignItems: 'center', gap: '20px', padding: '18px 24px' }}
          onClick={() => onPowerAction('restart')}
          tabIndex={61}
          role="button"
        >
          <div className="power-icon" style={{ fontSize: '24px', color: 'var(--deck-amber)' }}>
            <RefreshCw size={24} />
          </div>
          <div className="power-info" style={{ flex: 1 }}>
            <div className="power-action-title">{t.power_restart_title}</div>
            <div className="power-action-desc">{t.power_restart_desc}</div>
          </div>
          <button className="deck-btn">{t.btn_restart}</button>
        </div>

        <div
          className="power-card focusable"
          style={{ display: 'flex', alignItems: 'center', gap: '20px', padding: '18px 24px' }}
          onClick={() => onPowerAction('shutdown')}
          tabIndex={62}
          role="button"
        >
          <div className="power-icon" style={{ fontSize: '24px', color: 'var(--deck-danger)' }}>
            <Power size={24} />
          </div>
          <div className="power-info" style={{ flex: 1 }}>
            <div className="power-action-title">{t.power_shutdown_title}</div>
            <div className="power-action-desc">{t.power_shutdown_desc}</div>
          </div>
          <button className="deck-btn deck-btn-danger">{t.btn_shutdown}</button>
        </div>
      </div>

      <div className="section-divider">
        <span className="divider-title">{t.controller_diag_title}</span>
      </div>

      <div className="gamepad-diagnostic-box" style={{ backgroundColor: 'var(--deck-card-bg)', border: '1px solid var(--deck-border)', borderRadius: '4px', padding: '16px 20px' }}>
        <div className="diag-header" style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '12px', fontSize: '12px' }}>
          <span style={{ color: '#ffffff', fontWeight: 600 }}>{gamepadTelemetry.name}</span>
          <span
            className={`diag-status ${gamepadTelemetry.connected ? 'connected' : ''}`}
            style={{
              fontFamily: 'var(--font-mono)',
              fontSize: '10px',
              padding: '2px 6px',
              borderRadius: '3px',
              color: gamepadTelemetry.connected ? 'var(--deck-green)' : 'var(--deck-amber)',
              backgroundColor: gamepadTelemetry.connected ? 'rgba(92, 191, 71, 0.12)' : 'rgba(229, 165, 36, 0.12)',
            }}
          >
            {gamepadTelemetry.connected ? 'ACTIVE' : 'STANDBY'}
          </span>
        </div>
        <div className="diag-bars" style={{ display: 'flex', gap: '24px' }}>
          <div className="diag-stick" style={{ flex: 1, backgroundColor: 'var(--deck-header-bg)', border: '1px solid var(--deck-border)', borderRadius: '4px', padding: '10px 14px', display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ fontSize: '10px', fontWeight: 700, color: 'var(--deck-text-muted)' }}>LEFT STICK</span>
            <span style={{ fontFamily: 'var(--font-mono)', fontSize: '11px', color: 'var(--deck-focus)' }}>
              X: {gamepadTelemetry.leftStick.x.toFixed(2)} | Y: {gamepadTelemetry.leftStick.y.toFixed(2)}
            </span>
          </div>
          <div className="diag-stick" style={{ flex: 1, backgroundColor: 'var(--deck-header-bg)', border: '1px solid var(--deck-border)', borderRadius: '4px', padding: '10px 14px', display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ fontSize: '10px', fontWeight: 700, color: 'var(--deck-text-muted)' }}>RIGHT STICK</span>
            <span style={{ fontFamily: 'var(--font-mono)', fontSize: '11px', color: 'var(--deck-focus)' }}>
              X: {gamepadTelemetry.rightStick.x.toFixed(2)} | Y: {gamepadTelemetry.rightStick.y.toFixed(2)}
            </span>
          </div>
        </div>
      </div>
    </section>
  );
}

