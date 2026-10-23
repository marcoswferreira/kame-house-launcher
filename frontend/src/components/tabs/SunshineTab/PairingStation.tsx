import { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { SunshineClientDevice } from '../../../types/models';
import { KeyRound, Smartphone, Trash2 } from 'lucide-react';

interface PairingStationProps {
  clients: SunshineClientDevice[];
  onPairPin: (pin: string) => Promise<boolean>;
  onUnpairClient: (uuid: string) => void;
  onUnpairAll: () => void;
}

export function PairingStation({
  clients,
  onPairPin,
  onUnpairClient,
  onUnpairAll,
}: PairingStationProps) {
  const { t } = useTranslation();
  const [pin, setPin] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const handleKeyClick = async (key: string) => {
    if (key === 'clear') {
      setPin('');
      return;
    }
    if (key === 'submit') {
      if (pin.length === 4) {
        setSubmitting(true);
        const success = await onPairPin(pin);
        setSubmitting(false);
        if (success) setPin('');
      }
      return;
    }
    if (pin.length < 4 && /^\d$/.test(key)) {
      const next = pin + key;
      setPin(next);
      if (next.length === 4) {
        setSubmitting(true);
        const success = await onPairPin(next);
        setSubmitting(false);
        if (success) setPin('');
      }
    }
  };

  return (
    <div>
      {/* 4-digit PIN Pairing Box */}
      <div className="sunshine-pin-box" style={{ backgroundColor: 'var(--deck-card-bg)', border: '1px solid var(--deck-border)', borderRadius: '4px', padding: '24px', display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: '24px' }}>
        <div className="pin-header" style={{ maxWidth: '520px', textAlign: 'center', marginBottom: '18px' }}>
          <div className="pin-badge" style={{ display: 'inline-block', fontSize: '10px', fontWeight: 700, letterSpacing: '0.08em', backgroundColor: 'rgba(229, 165, 36, 0.15)', color: 'var(--deck-amber)', border: '1px solid rgba(229, 165, 36, 0.35)', padding: '2px 8px', borderRadius: '3px', marginBottom: '8px' }}>
            {t.pin_badge}
          </div>
          <h2 className="pin-title" style={{ fontSize: '18px', fontWeight: 600, color: '#ffffff', marginBottom: '6px' }}>{t.pin_box_title}</h2>
          <p className="pin-desc" style={{ fontSize: '12px', color: 'var(--deck-text-secondary)', lineHeight: 1.4 }}>{t.pin_box_desc}</p>
        </div>

        <div className="pin-digits-display">
          {[0, 1, 2, 3].map((idx) => {
            const digit = pin[idx] || '-';
            const isFilled = idx < pin.length;
            return (
              <div key={idx} className={`digit-box ${isFilled ? 'filled' : ''}`}>
                {digit}
              </div>
            );
          })}
        </div>

        <div className="pin-keypad">
          {['1', '2', '3', '4', '5', '6', '7', '8', '9'].map((digit) => (
            <button
              key={digit}
              className="keypad-btn focusable"
              onClick={() => handleKeyClick(digit)}
              tabIndex={30 + parseInt(digit, 10)}
            >
              {digit}
            </button>
          ))}
          <button className="keypad-btn btn-clear focusable" onClick={() => handleKeyClick('clear')} tabIndex={40}>
            {t.btn_pin_clear}
          </button>
          <button className="keypad-btn focusable" onClick={() => handleKeyClick('0')} tabIndex={41}>
            0
          </button>
          <button
            className="keypad-btn btn-submit focusable"
            onClick={() => handleKeyClick('submit')}
            disabled={submitting}
            tabIndex={42}
          >
            {submitting ? '...' : t.btn_pin_pair}
          </button>
        </div>
      </div>

      {/* Paired Moonlight Clients Table */}
      <div className="section-divider">
        <span className="divider-title">{t.paired_clients_title}</span>
        {clients.length > 0 && (
          <button className="deck-btn deck-btn-danger focusable" style={{ padding: '4px 10px', fontSize: '11px' }} onClick={onUnpairAll}>
            <Trash2 size={12} />
            <span>{t.btn_unpair_all}</span>
          </button>
        )}
      </div>

      <div className="table-container">
        <table className="deck-table">
          <thead>
            <tr>
              <th>{t.th_client_name}</th>
              <th>{t.th_client_id}</th>
              <th>{t.th_client_ip}</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {clients.length === 0 ? (
              <tr>
                <td colSpan={4} className="table-empty" style={{ textAlign: 'center', padding: '24px 16px', color: 'var(--deck-text-muted)' }}>
                  <Smartphone size={20} style={{ display: 'block', margin: '0 auto 8px auto', opacity: 0.5 }} />
                  {t.no_clients_paired}
                </td>
              </tr>
            ) : (
              clients.map((client) => (
                <tr key={client.uuid}>
                  <td style={{ color: '#ffffff', fontWeight: 600 }}>{client.name}</td>
                  <td className="mono-cell">{client.uuid}</td>
                  <td className="mono-cell">{client.ip || '192.168.x.x'}</td>
                  <td>
                    <button
                      className="deck-btn deck-btn-danger focusable"
                      style={{ padding: '4px 8px', fontSize: '11px' }}
                      onClick={() => onUnpairClient(client.uuid)}
                    >
                      <Trash2 size={11} />
                      <span>{t.btn_unpair}</span>
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

