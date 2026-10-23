export function FooterLegend() {
  const currentPort =
    typeof window !== 'undefined' && window.location.port
      ? window.location.port
      : '48080';

  return (
    <footer className="deck-footer">
      <div className="footer-telemetry">
        <span>PORT {currentPort}</span>
        <span>VDF READY</span>
        <span>KAME HOUSE LAUNCHER</span>
      </div>
    </footer>
  );
}
