import { useTranslation } from '../../../i18n';
import { Radio, ShieldCheck, Cpu } from 'lucide-react';

interface StandbyViewProps {
  onEnableSunshine: () => void;
}

export function StandbyView({ onEnableSunshine }: StandbyViewProps) {
  const { t } = useTranslation();

  return (
    <div className="sunshine-standby-hero">
      <div className="standby-badge">{t.sunshine_optin_badge}</div>
      <Radio size={42} style={{ color: 'var(--deck-focus)', marginBottom: '16px' }} />
      <h2 className="standby-title">{t.sunshine_optin_title}</h2>
      <p className="standby-desc">{t.sunshine_optin_desc}</p>

      <div className="standby-note">
        <ShieldCheck size={14} style={{ display: 'inline', marginRight: '6px', verticalAlign: '-2px', color: 'var(--deck-green)' }} />
        <span>{t.sunshine_optin_note}</span>
      </div>

      <button className="deck-btn deck-btn-primary focusable" onClick={onEnableSunshine} tabIndex={30} style={{ padding: '12px 24px', fontSize: '14px' }}>
        <Cpu size={16} />
        <span>{t.btn_enable_sunshine}</span>
      </button>
    </div>
  );
}

