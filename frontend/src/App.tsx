import { useState, useEffect, useRef } from 'react';
import { useTranslation } from './i18n';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { FooterLegend } from './components/FooterLegend';
import { OverviewTab } from './components/tabs/OverviewTab';
import { SteamBridgeTab } from './components/tabs/SteamBridgeTab';
import { SunshineTab } from './components/tabs/SunshineTab/SunshineTab';
import { DisplayTab } from './components/tabs/DisplayTab';
import { PowerTab } from './components/tabs/PowerTab';
import {
  StatusResponse,
  Game,
  SunshineConfig,
  SunshineApp,
  SunshineClientDevice,
} from './types/models';

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

  // Sunshine state
  const [sunshineConfig, setSunshineConfig] = useState<SunshineConfig>({});
  const [sunshineApps, setSunshineApps] = useState<SunshineApp[]>([]);
  const [sunshineClients, setSunshineClients] = useState<SunshineClientDevice[]>([]);
  const [sunshineLogs, setSunshineLogs] = useState<string>('');

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

  const fetchSunshineData = async () => {
    try {
      const [cfgRes, appsRes, clientsRes, logsRes] = await Promise.allSettled([
        fetch('/api/sunshine/config'),
        fetch('/api/sunshine/apps'),
        fetch('/api/sunshine/clients'),
        fetch('/api/sunshine/logs'),
      ]);

      if (cfgRes.status === 'fulfilled' && cfgRes.value.ok) {
        setSunshineConfig(await cfgRes.value.json());
      }
      if (appsRes.status === 'fulfilled' && appsRes.value.ok) {
        const data = await appsRes.value.json();
        setSunshineApps(data.apps || []);
      }
      if (clientsRes.status === 'fulfilled' && clientsRes.value.ok) {
        setSunshineClients(await clientsRes.value.json());
      }
      if (logsRes.status === 'fulfilled' && logsRes.value.ok) {
        setSunshineLogs(await logsRes.value.text());
      }
    } catch (_) {}
  };

  useEffect(() => {
    fetchStatus();
    fetchGames();
    fetchSunshineData();
  }, []);

  // Quick Action Handlers
  const handleSyncSteam = async () => {
    try {
      showToast(lang === 'pt' ? 'Exportando atalhos para a Steam...' : 'Exporting shortcuts to Steam...', 'info');
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

  const handleScanGames = async (paths?: string[]) => {
    try {
      showToast(lang === 'pt' ? 'Varrendo diretórios de jogos e ROMs...' : 'Scanning game and ROM directories...', 'info');
      const res = await fetch('/api/scan', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ paths }),
      });
      const data = await res.json();
      if (res.ok) {
        const msg = t.toast_scanned.replace('{count}', String(data.added || 0));
        showToast(msg, 'success');
        fetchGames();
        fetchStatus();
      } else {
        showToast(data.error || 'Scan failed', 'error');
      }
    } catch (e) {
      showToast('Error connecting to scanner', 'error');
    }
  };

  const handleUpdateScanDirectories = async (paths: string[]) => {
    try {
      await fetch('/api/scan/paths', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ paths }),
      });
      setStatus((prev) => (prev ? { ...prev, scan_directories: paths } : null));
      showToast(t.toast_paths_saved, 'success');
    } catch (_) {
      showToast('Failed to update scan directories.', 'error');
    }
  };

  const handleSyncSunshine = async () => {
    try {
      showToast(lang === 'pt' ? 'Sincronizando apps.json do Sunshine...' : 'Syncing Sunshine apps.json...', 'info');
      const res = await fetch('/api/sunshine/sync', { method: 'POST' });
      const data = await res.json();
      if (res.ok) {
        const msg = t.toast_sunshine_synced.replace('{count}', String(data.count || 0));
        showToast(msg, 'success');
        fetchSunshineData();
      } else {
        showToast(data.error || 'Sunshine sync failed', 'error');
      }
    } catch (e) {
      showToast('Error syncing apps.json', 'error');
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

  // Sunshine Handlers
  const handleToggleSunshine = async (enable: boolean) => {
    try {
      const res = await fetch('/api/sunshine/toggle', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled: enable }),
      });
      if (res.ok) {
        showToast(enable ? t.toast_sunshine_enabled : t.toast_sunshine_disabled, 'success');
        fetchStatus();
        if (enable) fetchSunshineData();
      }
    } catch (_) {
      // Optimistic local update
      setStatus((prev) => (prev ? { ...prev, sunshine_enabled: enable } : null));
      showToast(enable ? t.toast_sunshine_enabled : t.toast_sunshine_disabled, 'success');
    }
  };

  const handlePairPin = async (pin: string): Promise<boolean> => {
    try {
      showToast(lang === 'pt' ? 'Pareando com Sunshine...' : 'Pairing with Sunshine...', 'info');
      const res = await fetch('/api/sunshine/pin', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pin }),
      });
      if (res.ok) {
        showToast(t.toast_pin_paired, 'success');
        fetchSunshineData();
        return true;
      } else {
        showToast(t.toast_pin_error, 'error');
        return false;
      }
    } catch (_) {
      showToast(t.toast_pin_error, 'error');
      return false;
    }
  };

  const handleUnpairClient = async (uuid: string) => {
    try {
      await fetch('/api/sunshine/clients/unpair', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ uuid }),
      });
      setSunshineClients((prev) => prev.filter((c) => c.uuid !== uuid));
      showToast('Client unpaired successfully.', 'success');
    } catch (_) {
      showToast('Failed to unpair client.', 'error');
    }
  };

  const handleUnpairAll = async () => {
    if (!confirm('Unpair all Moonlight clients?')) return;
    try {
      await fetch('/api/sunshine/clients/unpair-all', { method: 'POST' });
      setSunshineClients([]);
      showToast('All clients unpaired.', 'success');
    } catch (_) {}
  };

  const handleSaveSunshineApp = async (app: SunshineApp) => {
    try {
      await fetch('/api/sunshine/apps', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(app),
      });
      showToast('Application saved to apps.json.', 'success');
      fetchSunshineData();
    } catch (_) {
      showToast('Failed to save app.', 'error');
    }
  };

  const handleDeleteSunshineApp = async (index: number) => {
    if (!confirm('Delete this app from Sunshine?')) return;
    try {
      await fetch(`/api/sunshine/apps/${index}`, { method: 'DELETE' });
      setSunshineApps((prev) => prev.filter((_, i) => i !== index));
      showToast('Application removed.', 'success');
    } catch (_) {}
  };

  const handleSaveSunshineConfig = async (patch: Partial<SunshineConfig>) => {
    try {
      await fetch('/api/sunshine/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(patch),
      });
      setSunshineConfig((prev) => ({ ...prev, ...patch }));
      showToast(t.toast_settings_saved, 'success');
    } catch (_) {
      showToast('Failed to save Sunshine configuration.', 'error');
    }
  };

  const handleRestartSunshine = async () => {
    try {
      await fetch('/api/sunshine/restart', { method: 'POST' });
      showToast('Sunshine daemon restarted.', 'success');
    } catch (_) {}
  };

  const handleResetDisplay = async () => {
    try {
      await fetch('/api/sunshine/reset-display-device-persistence', { method: 'POST' });
      showToast('Virtual display device persistence reset.', 'success');
    } catch (_) {}
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
        showToast(t.toast_power_dispatched.replace('{action}', actionLabel), 'success');
      }
    } catch (_) {
      showToast('Failed to dispatch power command.', 'error');
    }
  };

  // Display & OS
  const handleUpdateLaunchBehavior = async (val: string) => {
    try {
      await fetch('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ launch_behavior: val }),
      });
      showToast(t.toast_settings_saved, 'success');
      fetchStatus();
    } catch (_) {}
  };

  const handleDownloadAutoLoginReg = () => {
    const regContent = `Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon]
"AutoAdminLogon"="1"
"DefaultUserName"="ConsoleUser"
"DefaultDomainName"=""
"DefaultPassword"=""
"Shell"="explorer.exe"
; Kame House Launcher - Living Room Console Profile
`;
    const blob = new Blob([regContent], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'kame_house_console_autologin.reg';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    showToast(t.toast_reg_downloaded, 'success');
  };

  // Navigation Helpers
  const tabsList = ['overview', 'steam-bridge', 'sunshine', 'display', 'power'];
  const nextTab = () => {
    const idx = tabsList.indexOf(activeTab);
    setActiveTab(tabsList[(idx + 1) % tabsList.length]);
  };
  const prevTab = () => {
    const idx = tabsList.indexOf(activeTab);
    setActiveTab(tabsList[(idx - 1 + tabsList.length) % tabsList.length]);
  };

  // Gamepad Loop
  useEffect(() => {
    let animId: number;

    const gamepadLoop = () => {
      const gamepads = navigator.getGamepads ? navigator.getGamepads() : [];
      let gp: Gamepad | null = null;
      for (let i = 0; i < gamepads.length; i++) {
        if (gamepads[i] && gamepads[i]!.connected) {
          gp = gamepads[i];
          break;
        }
      }

      if (gp) {
        if (!isGamepadActive) {
          setIsGamepadActive(true);
        }

        const lx = gp.axes[0] || 0;
        const ly = gp.axes[1] || 0;
        const rx = gp.axes[2] || 0;
        const ry = gp.axes[3] || 0;

        setGamepadTelemetry({
          name: gp.id || 'Standard Gamepad',
          connected: true,
          leftStick: { x: lx, y: ly },
          rightStick: { x: rx, y: ry },
        });

        // Spatial focus navigation
        const threshold = 0.5;
        const dpadUp = gp.buttons[12]?.pressed;
        const dpadDown = gp.buttons[13]?.pressed;
        const dpadLeft = gp.buttons[14]?.pressed;
        const dpadRight = gp.buttons[15]?.pressed;

        const stickUp = ly < -threshold && prevAxes.current[1] >= -threshold;
        const stickDown = ly > threshold && prevAxes.current[1] <= threshold;
        const stickLeft = lx < -threshold && prevAxes.current[0] >= -threshold;
        const stickRight = lx > threshold && prevAxes.current[0] <= threshold;

        if ((dpadDown || stickDown) && !prevButtons.current[13]) {
          moveFocus('down');
        } else if ((dpadUp || stickUp) && !prevButtons.current[12]) {
          moveFocus('up');
        } else if ((dpadRight || stickRight) && !prevButtons.current[15]) {
          moveFocus('right');
        } else if ((dpadLeft || stickLeft) && !prevButtons.current[14]) {
          moveFocus('left');
        }

        // Button A (0): Click
        if (gp.buttons[0]?.pressed && !prevButtons.current[0]) {
          const el = document.activeElement as HTMLElement | null;
          if (el && typeof el.click === 'function') {
            el.click();
          }
        }

        // Button X (2): Quick Sync
        if (gp.buttons[2]?.pressed && !prevButtons.current[2]) {
          handleSyncSteam();
        }

        // Button Y (3): Language Toggle
        if (gp.buttons[3]?.pressed && !prevButtons.current[3]) {
          toggleLang();
        }

        // LB (4): Prev Tab
        if (gp.buttons[4]?.pressed && !prevButtons.current[4]) {
          prevTab();
        }

        // RB (5): Next Tab
        if (gp.buttons[5]?.pressed && !prevButtons.current[5]) {
          nextTab();
        }

        prevAxes.current = [lx, ly];
        prevButtons.current = gp.buttons.map((b) => b.pressed);
      } else {
        if (isGamepadActive) {
          setIsGamepadActive(false);
          setGamepadTelemetry({
            name: 'No Gamepad Detected (Press any button)',
            connected: false,
            leftStick: { x: 0, y: 0 },
            rightStick: { x: 0, y: 0 },
          });
        }
      }

      animId = requestAnimationFrame(gamepadLoop);
    };

    animId = requestAnimationFrame(gamepadLoop);
    return () => cancelAnimationFrame(animId);
  }, [isGamepadActive, activeTab]);

  const moveFocus = (dir: 'up' | 'down' | 'left' | 'right') => {
    const focusables = Array.from(
      document.querySelectorAll<HTMLElement>('.focusable:not([disabled])')
    ).filter((el) => el.offsetParent !== null);

    if (focusables.length === 0) return;
    const current = document.activeElement as HTMLElement | null;
    if (!current || !focusables.includes(current)) {
      focusables[0].focus();
      return;
    }

    const currentRect = current.getBoundingClientRect();
    let best: HTMLElement | null = null;
    let minDist = Infinity;

    focusables.forEach((target) => {
      if (target === current) return;
      const targetRect = target.getBoundingClientRect();
      const dx = targetRect.left + targetRect.width / 2 - (currentRect.left + currentRect.width / 2);
      const dy = targetRect.top + targetRect.height / 2 - (currentRect.top + currentRect.height / 2);

      let isAligned = false;
      if (dir === 'down' && dy > 10) isAligned = true;
      if (dir === 'up' && dy < -10) isAligned = true;
      if (dir === 'right' && dx > 10) isAligned = true;
      if (dir === 'left' && dx < -10) isAligned = true;

      if (isAligned) {
        const dist = dir === 'down' || dir === 'up'
          ? Math.abs(dy) + Math.abs(dx) * 1.5
          : Math.abs(dx) + Math.abs(dy) * 1.5;
        if (dist < minDist) {
          minDist = dist;
          best = target;
        }
      }
    });

    if (best) {
      (best as HTMLElement).focus();
      (best as HTMLElement).scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    }
  };

  // Keyboard Shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'x' || e.key === 'X') {
        if ((e.target as HTMLElement).tagName !== 'INPUT') {
          handleSyncSteam();
        }
      } else if (e.key === 'y' || e.key === 'Y') {
        if ((e.target as HTMLElement).tagName !== 'INPUT') {
          toggleLang();
        }
      } else if (e.key === 'PageDown') {
        nextTab();
      } else if (e.key === 'PageUp') {
        prevTab();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [activeTab]);

  return (
    <div className="deck-window">
      <Header status={status} isGamepadActive={isGamepadActive} />

      <div className="deck-body">
        <Sidebar
          activeTab={activeTab}
          onSelectTab={setActiveTab}
          sunshineEnabled={status?.sunshine_enabled}
        />

        <main className="deck-viewport">
          {activeTab === 'overview' && (
            <OverviewTab
              status={status}
              onNavigateTab={setActiveTab}
              onSyncSteam={handleSyncSteam}
              onScanGames={handleScanGames}
              onSyncSunshine={handleSyncSunshine}
            />
          )}

          {activeTab === 'steam-bridge' && (
            <SteamBridgeTab
              games={games}
              scanDirectories={status?.scan_directories || []}
              onSyncSteam={handleSyncSteam}
              onScanGames={handleScanGames}
              onUpdateScanDirectories={handleUpdateScanDirectories}
              onLaunchGame={handleLaunchGame}
            />
          )}

          {activeTab === 'sunshine' && (
            <SunshineTab
              sunshineEnabled={Boolean(status?.sunshine_enabled)}
              onToggleSunshine={handleToggleSunshine}
              config={sunshineConfig}
              apps={sunshineApps}
              clients={sunshineClients}
              logs={sunshineLogs}
              onPairPin={handlePairPin}
              onUnpairClient={handleUnpairClient}
              onUnpairAll={handleUnpairAll}
              onSyncAppsJson={handleSyncSunshine}
              onSaveApp={handleSaveSunshineApp}
              onDeleteApp={handleDeleteSunshineApp}
              onSaveConfig={handleSaveSunshineConfig}
              onRefreshLogs={fetchSunshineData}
              onRestartSunshine={handleRestartSunshine}
              onResetDisplay={handleResetDisplay}
            />
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

      {/* Toast Notifications */}
      <div className="deck-toast-container">
        {toasts.map((toast) => (
          <div key={toast.id} className={`deck-toast toast-${toast.type}`}>
            <span>{toast.type === 'success' ? '✓' : toast.type === 'error' ? '⚠' : 'ℹ'}</span>
            <span>{toast.message}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

