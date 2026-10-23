import { useState, useEffect } from 'react';
import { useTranslation } from './i18n';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { FooterLegend } from './components/FooterLegend';
import { OverviewTab } from './components/tabs/OverviewTab';
import { SteamBridgeTab } from './components/tabs/SteamBridgeTab';
import { DualSenseTab } from './components/tabs/DualSenseTab/DualSenseTab';
import { UtilitiesTab } from './components/tabs/UtilitiesTab/UtilitiesTab';
import { StatusResponse, Game } from './types/models';

interface Toast {
  id: number;
  message: string;
  type: 'info' | 'success' | 'error';
}

const TABS_LIST = ['overview', 'steam-bridge', 'dualsense', 'utilities'];

export function App() {
  const { t, lang, toggleLang } = useTranslation();
  const [activeTab, setActiveTab] = useState('overview');
  const [status, setStatus] = useState<StatusResponse | null>(null);
  const [games, setGames] = useState<Game[]>([]);
  const [toasts, setToasts] = useState<Toast[]>([]);
  const [isCloseModalOpen, setIsCloseModalOpen] = useState(false);

  // Gamepad state
  const [isGamepadActive, setIsGamepadActive] = useState(false);

  const showToast = (message: string, type: 'info' | 'success' | 'error' = 'info') => {
    const id = Date.now() + Math.random();
    setToasts((prev) => [...prev, { id, message, type }]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 4000);
  };

  // Initial Data Fetching
  const fetchStatus = async () => {
    try {
      const res = await fetch('/api/status');
      if (res.ok) {
        const data: StatusResponse = await res.json();
        setStatus(data);
      }
    } catch (_) {}
  };

  const fetchGames = async () => {
    try {
      const res = await fetch('/api/games');
      if (res.ok) {
        const data: Game[] = await res.json();
        setGames(data);
      }
    } catch (_) {}
  };

  useEffect(() => {
    fetchStatus();
    fetchGames();

    const handlePageHide = () => {
      if (navigator.sendBeacon) {
        navigator.sendBeacon('/api/window/native-close', '{}');
      }
    };
    window.addEventListener('pagehide', handlePageHide);
    return () => window.removeEventListener('pagehide', handlePageHide);
  }, []);

  const handleToggleDebugConsole = async () => {
    try {
      const res = await fetch('/api/window/toggle-console', { method: 'POST' });
      if (res.ok) {
        const data = await res.json();
        await fetchStatus();
        showToast(
          data.debug_console_visible
            ? lang === 'pt'
              ? 'Terminal de Debug aberto.'
              : 'Debug Terminal opened.'
            : lang === 'pt'
              ? 'Terminal de Debug ocultado.'
              : 'Debug Terminal hidden.',
          'info'
        );
      }
    } catch (_) {}
  };

  const handleMinimizeToTray = async () => {
    setIsCloseModalOpen(false);
    try {
      await fetch('/api/window/minimize', { method: 'POST' });
    } catch (_) {}
    showToast(
      lang === 'pt'
        ? 'Minimizado para a bandeja do sistema (System Tray).'
        : 'Minimized to System Tray.',
      'info'
    );
    setTimeout(() => {
      window.close();
    }, 150);
  };

  const handleQuitApplication = async () => {
    setIsCloseModalOpen(false);
    try {
      await fetch('/api/window/quit', { method: 'POST' });
    } catch (_) {}
    setTimeout(() => {
      window.close();
    }, 100);
  };

  // Quick Action Handlers
  const handleSyncSteam = async () => {
    try {
      showToast(lang === 'pt' ? 'Exportando atalhos para Steam...' : 'Syncing shortcuts into Steam...', 'info');
      const res = await fetch('/api/steam/sync', { method: 'POST' });
      const data = await res.json();
      if (res.ok) {
        const msg = t.toast_steam_synced.replace('{count}', String(data.count || 0));
        showToast(msg, 'success');
        fetchStatus();
      } else {
        showToast(data.error || 'Steam sync failed', 'error');
      }
    } catch (e) {
      showToast('Error syncing shortcuts.vdf', 'error');
    }
  };

  const handleScanGames = async (customPaths?: string[]) => {
    try {
      showToast(lang === 'pt' ? 'Escaneando pastas de jogos...' : 'Scanning game folders...', 'info');
      const res = await fetch('/api/scan', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ paths: customPaths }),
      });
      const data = await res.json();
      if (res.ok) {
        const msg = t.toast_scanned.replace('{count}', String(data.count || 0));
        showToast(msg, 'success');
        fetchGames();
        fetchStatus();
      } else {
        showToast(data.error || 'Scan failed', 'error');
      }
    } catch (e) {
      showToast('Error scanning directories', 'error');
    }
  };

  const handleUpdateScanDirectories = async (paths: string[]) => {
    try {
      const res = await fetch('/api/scan/paths', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ paths }),
      });
      if (res.ok) {
        showToast(t.toast_paths_saved, 'success');
        fetchStatus();
      } else {
        showToast('Failed to update scan directories.', 'error');
      }
    } catch (e) {
      showToast('Failed to update scan directories.', 'error');
    }
  };

  const handleLaunchGame = async (id: string) => {
    try {
      const res = await fetch('/api/launch', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id }),
      });
      if (res.ok) {
        showToast('Game launched via supervisor.', 'success');
      } else {
        showToast('Launch failed.', 'error');
      }
    } catch (_) {
      showToast('Error connecting to launcher daemon.', 'error');
    }
  };

  // Passive Gamepad Telemetry (No button hijacking or tab switching)
  useEffect(() => {
    let animId: number;

    const pollGamepad = () => {
      const gamepads = navigator.getGamepads ? navigator.getGamepads() : [];
      let foundActive = false;

      for (let i = 0; i < gamepads.length; i++) {
        const gp = gamepads[i];
        if (!gp) continue;

        foundActive = true;
        setIsGamepadActive(true);
        break;
      }

      if (!foundActive) {
        setIsGamepadActive(false);
      }

      animId = requestAnimationFrame(pollGamepad);
    };

    animId = requestAnimationFrame(pollGamepad);
    return () => cancelAnimationFrame(animId);
  }, []);

  // Keyboard navigation shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (['INPUT', 'TEXTAREA', 'SELECT'].includes((e.target as HTMLElement)?.tagName)) {
        return;
      }

      if (e.key === 'q' || e.key === 'Q') {
        setActiveTab((curr) => {
          const idx = TABS_LIST.indexOf(curr);
          return TABS_LIST[(idx - 1 + TABS_LIST.length) % TABS_LIST.length];
        });
      } else if (e.key === 'e' || e.key === 'E') {
        setActiveTab((curr) => {
          const idx = TABS_LIST.indexOf(curr);
          return TABS_LIST[(idx + 1) % TABS_LIST.length];
        });
      } else if (e.key === 'y' || e.key === 'Y') {
        toggleLang();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [toggleLang]);

  return (
    <div className="deck-window">
      <Header
        status={status}
        isGamepadActive={isGamepadActive}
        onToggleDebugConsole={handleToggleDebugConsole}
        onRequestClose={() => setIsCloseModalOpen(true)}
      />

      <div className="deck-body">
        <Sidebar
          activeTab={activeTab}
          onSelectTab={setActiveTab}
          dualsenseExtensionEnabled={status?.dualsense_extension_enabled}
          dualsenseConnected={status?.dualsense_connected}
        />

        <main className="deck-viewport">
          {activeTab === 'overview' && (
            <OverviewTab
              status={status}
              onNavigateTab={setActiveTab}
              onSyncSteam={handleSyncSteam}
              onScanGames={() => handleScanGames()}
            />
          )}

          {activeTab === 'steam-bridge' && (
            <SteamBridgeTab
              games={games}
              scanDirectories={status?.scan_directories || []}
              onSyncSteam={handleSyncSteam}
              onScanGames={() => handleScanGames()}
              onUpdateScanDirectories={handleUpdateScanDirectories}
              onLaunchGame={handleLaunchGame}
            />
          )}

          {activeTab === 'dualsense' && (
            <DualSenseTab
              onShowToast={(msg) => showToast(msg, 'success')}
              onRefreshAppStatus={fetchStatus}
            />
          )}

          {activeTab === 'utilities' && (
            <UtilitiesTab onShowToast={(msg) => showToast(msg, 'info')} />
          )}
        </main>
      </div>

      <FooterLegend />

      {/* Close / Minimize to System Tray Confirmation Modal */}
      {isCloseModalOpen && (
        <div
          style={{
            position: 'fixed',
            inset: 0,
            backgroundColor: 'rgba(10, 14, 20, 0.82)',
            backdropFilter: 'blur(6px)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            zIndex: 9999,
          }}
          onClick={() => setIsCloseModalOpen(false)}
        >
          <div
            style={{
              background: 'var(--deck-surface, #161b24)',
              border: '1px solid rgba(255, 255, 255, 0.12)',
              borderRadius: '14px',
              padding: '24px',
              width: '100%',
              maxWidth: '460px',
              boxShadow: '0 20px 50px rgba(0, 0, 0, 0.6)',
            }}
            onClick={(e) => e.stopPropagation()}
          >
            <h3 style={{ margin: '0 0 10px 0', fontSize: '18px', color: '#fff' }}>
              {lang === 'pt' ? 'Fechar Kame House Launcher' : 'Close Kame House Launcher'}
            </h3>
            <p style={{ margin: '0 0 20px 0', fontSize: '13.5px', lineHeight: 1.5, color: 'var(--deck-text-muted, #9ca3af)' }}>
              {lang === 'pt'
                ? 'Deseja minimizar o Kame House para continuar online na bandeja do sistema (System Tray) ou encerrar completamente a aplicação?'
                : 'Do you want to minimize Kame House to stay online in the System Tray or quit the application completely?'}
            </p>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <button
                className="btn-primary focusable"
                onClick={handleMinimizeToTray}
                style={{ width: '100%', justifyContent: 'center', padding: '10px 16px' }}
              >
                {lang === 'pt'
                  ? '🗕 Minimizar para a Bandeja (System Tray)'
                  : '🗕 Minimize to System Tray'}
              </button>
              <button
                className="btn-secondary focusable"
                onClick={handleQuitApplication}
                style={{
                  width: '100%',
                  justifyContent: 'center',
                  padding: '10px 16px',
                  borderColor: 'rgba(239, 68, 68, 0.45)',
                  color: '#f87171',
                }}
              >
                {lang === 'pt' ? '⏻ Encerrar Aplicação' : '⏻ Quit Application'}
              </button>
              <button
                className="btn-secondary focusable"
                onClick={() => setIsCloseModalOpen(false)}
                style={{ width: '100%', justifyContent: 'center', padding: '8px 16px' }}
              >
                {lang === 'pt' ? 'Cancelar' : 'Cancel'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Steam Deck Toast System */}
      <aside className="toast-container" aria-live="polite">
        {toasts.map((toast) => (
          <div key={toast.id} className={`deck-toast ${toast.type}`}>
            <span className="toast-dot" />
            <span className="toast-msg">{toast.message}</span>
          </div>
        ))}
      </aside>
    </div>
  );
}
