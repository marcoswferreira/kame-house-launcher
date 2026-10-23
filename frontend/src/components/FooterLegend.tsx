import { useTranslation } from '../i18n';

export function FooterLegend() {
  const { t } = useTranslation();

  return (
    <footer className="deck-footer">
      <div className="controller-legend">
        <div className="legend-chip">
          <span className="deck-btn-badge badge-a">A</span>
          <span>{t.prompt_select}</span>
        </div>
        <div className="legend-chip">
          <span className="deck-btn-badge badge-b">B</span>
          <span>{t.prompt_back}</span>
        </div>
        <div className="legend-chip">
          <span className="deck-btn-badge badge-x">X</span>
          <span>{t.prompt_sync}</span>
        </div>
        <div className="legend-chip">
          <span className="deck-btn-badge badge-y">Y</span>
          <span>{t.prompt_language}</span>
        </div>
        <div className="legend-chip">
          <span className="deck-bumper-small">LB</span>
          <span className="deck-bumper-small">RB</span>
          <span>{t.prompt_tabs}</span>
        </div>
      </div>

      <div className="footer-telemetry">
        <span>PORT 48080</span>
        <span>VDF READY</span>
        <span>KAME HOUSE LAUNCHER</span>
      </div>
    </footer>
  );
}

