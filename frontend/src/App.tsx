import { useState, useEffect, useRef } from 'react';
import { useTranslation } from './i18n';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { FooterLegend } from './components/FooterLegend';
import { OverviewTab } from './components/tabs/OverviewTab';
import { SteamBridgeTab } from './components/tabs/SteamBridgeTab';
import { DualSenseTab } from './components/tabs/DualSenseTab/DualSenseTab';
import { DisplayTab } from './components/tabs/DisplayTab';
import { PowerTab } from './components/tabs/PowerTab';
import { StatusResponse, Game } from './types/models';

interface Toast {
  id: number;
  message: string;
  type: 'info' | 'success' | 'error';
}

export function App() {
  const { t, lang, toggleLang } = useTranslation();
  const [activeTab, setActiveTab] = useState('overview');
  const [status, setStatus] = useState<StatusResponse | null>(null);
  const [games, setGames] = useState<Game[]>([]);
  const [toasts, setToasts] = useState<Toast[]>([]);

  // Gamepad state
  const [isGamepadActive, setIsGamepadActive] = useState(false);
  const [gamepadTelemetry, setGamepadTelemetry] = useState({
    name: 'No Gamepad Detected (Press any button)',
    connected: false,
    leftStick: { x: 0, y: 0 },
    rightStick: { x: 0, y: 0 },
  });

  const prevAxes = useRef<[number, number]>([0, 0]);
  const prevButtons = useRef<boolean[]>([]);

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
  }, []);

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

  // Power actions
  const handlePowerAction = async (action: 'sleep' | 'restart' | 'shutdown') => {
    const actionLabel = t[`power_${action}_title` as keyof typeof t] || action;
    const confirmMsg = lang === 'pt'
      ? `Confirmar ação de energia: ${actionLabel}?`
      : `Confirm power action: ${actionLabel}?`;

    if (!confirm(confirmMsg)) return;

    try {
      const res = await fetch('/api/power', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action }),
      });
      if (res.ok) {
        const msg = t.toast_power_dispatched.replace('{action}', action);
        showToast(msg, 'success');
      } else {
        showToast('Power command failed', 'error');
      }
    } catch (_) {
      showToast('Failed to send power command', 'error');
    }
  };

  // Display & OS settings
  const handleUpdateLaunchBehavior = async (behavior: string) => {
    try {
      const res = await fetch('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ launch_behavior: behavior }),
      });
      if (res.ok) {
        showToast(t.toast_settings_saved, 'success');
        fetchStatus();
      }
    } catch (_) {}
  };

  const handleDownloadAutoLoginReg = () => {
    const regContent = `Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon]
"AutoAdminLogon"="1"
"DefaultUserName"="Gamer"
"DefaultPassword"=""
"DefaultDomainName"=""
`;
    const blob = new Blob([regContent], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'kame-house-autologon.reg';
    a.click();
    URL.revokeObjectURL(url);
    showToast(t.toast_reg_downloaded, 'success');
  };

  // 10-Foot Gamepad Navigation Engine
  const tabsList = ['overview', 'steam-bridge', 'dualsense', 'display', 'power'];

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
        setGamepadTelemetry({
          name: gp.id,
          connected: gp.connected,
          leftStick: {
            x: Math.round(gp.axes[0] * 100),
            y: Math.round(gp.axes[1] * 100),
          },
          rightStick: {
            x: Math.round((gp.axes[2] || 0) * 100),
            y: Math.round((gp.axes[3] || 0) * 100),
          },
        });

        const bLB = gp.buttons[4]?.pressed;
        const bRB = gp.buttons[5]?.pressed;
        const bY = gp.buttons[3]?.pressed;
        const bX = gp.buttons[2]?.pressed;

        const wasLB = prevButtons.current[4] || false;
        const wasRB = prevButtons.current[5] || false;
        const wasY = prevButtons.current[3] || false;
        const wasX = prevButtons.current[2] || false;

        if (bLB && !wasLB) {
          setActiveTab((curr) => {
            const idx = tabsList.indexOf(curr);
            const prevIdx = (idx - 1 + tabsList.length) % tabsList.length;
            return tabsList[prevIdx];
          });
        }
        if (bRB && !wasRB) {
          setActiveTab((curr) => {
            const idx = tabsList.indexOf(curr);
            const nextIdx = (idx + 1) % tabsList.length;
            return tabsList[nextIdx];
          });
        }

        if (bY && !wasY) {
          toggleLang();
        }

        if (bX && !wasX) {
          handleSyncSteam();
        }

        prevButtons.current = gp.buttons.map((b) => b.pressed);
        prevAxes.current = [gp.axes[0], gp.axes[1]];
        break;
      }

      if (!foundActive) {
        setIsGamepadActive(false);
      }

      animId = requestAnimationFrame(pollGamepad);
    };

    animId = requestAnimationFrame(pollGamepad);
    return () => cancelAnimationFrame(animId);
  }, [tabsList, toggleLang]);

  // Keyboard navigation shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (['INPUT', 'TEXTAREA', 'SELECT'].includes((e.target as HTMLElement)?.tagName)) {
        return;
      }

      if (e.key === 'q' || e.key === 'Q') {
        setActiveTab((curr) => {
          const idx = tabsList.indexOf(curr);
          return tabsList[(idx - 1 + tabsList.length) % tabsList.length];
        });
      } else if (e.key === 'e' || e.key === 'E') {
        setActiveTab((curr) => {
          const idx = tabsList.indexOf(curr);
          return tabsList[(idx + 1) % tabsList.length];
        });
      } else if (e.key === 'y' || e.key === 'Y') {
        toggleLang();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [tabsList, toggleLang]);

  return (
    <div className="deck-window">
      <Header status={status} isGamepadActive={isGamepadActive} />

      <div className="deck-body">
        <Sidebar
          activeTab={activeTab}
          onSelectTab={setActiveTab}
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
            <DualSenseTab onShowToast={(msg) => showToast(msg, 'success')} />
          )}

          {activeTab === 'display' && (
            <DisplayTab
              status={status}
              onUpdateLaunchBehavior={handleUpdateLaunchBehavior}
              onDownloadAutoLoginReg={handleDownloadAutoLoginReg}
            />
          )}

          {activeTab === 'power' && (
            <PowerTab
              onPowerAction={handlePowerAction}
              gamepadTelemetry={gamepadTelemetry}
            />
          )}
        </main>
      </div>

      <FooterLegend />

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
