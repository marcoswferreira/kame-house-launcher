# Console Mode Guide: Transforming Your PC into a Living Room Console 📺🎮

The **Console Mode** in Kame House Launcher is designed to turn any standard PC into a dedicated home gaming console, stripping away desktop operating system frictions.

<p align="center">
  <b>English</b> · <a href="CONSOLE_MODE.pt-BR.md">Português (Brasil)</a>
</p>

---

## 1. Direct Startup (Cold Boot Experience)

To achieve a true console feel, the PC should boot straight into the launcher interface without displaying password prompts, taskbars, or desktop icons.

### 1.1 Password-Free Auto-Login
- **On Windows:**
  The `launcher-console` module generates registry directives using official `AutoAdminLogon` keys:
  ```reg
  [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon]
  "AutoAdminLogon"="1"
  "DefaultUserName"="YourUser"
  "DefaultDomainName"="."
  ```
- **On Linux:**
  Automated desktop manager configuration via `greetd`, `gdm`, or `lightdm` using `autologin-user=<username>`.

### 1.2 System Autostart
- **Linux:** Generates a `kame-house-launcher.desktop` entry in `~/.config/autostart/` passing the `--fullscreen` flag.
- **Windows:** Registers into `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` to launch maximized.

### 1.3 Kiosk / Alternative Shell Mode
On Windows, Kame House Launcher can replace `explorer.exe` as the default user shell, ensuring that Windows desktop icons, search bars, and taskbars are completely hidden during boot and gaming.

---

## 2. Living Room Display & Audio Optimizations

### 2.1 High Refresh Rate TV Presets
When connecting the PC to modern living room TVs via HDMI 2.1:
- **4K @ 120Hz:** Full 3840x2160 resolution at 120Hz for OLED / QLED TVs.
- **1440p @ 120Hz:** Optimal balance of crisp resolution and high framerates for midrange GPUs.
- **1080p @ 120Hz / 60Hz:** Standard for legacy monitors and older TVs.

### 2.2 High Dynamic Range (HDR) Toggle
Console Mode provides a one-click HDR toggle, ensuring deep blacks and vibrant colors on HDR-capable panels.

### 2.3 Automatic Audio Routing
Ensures game audio is automatically routed to the intended output (TV HDMI, Soundbar via HDMI eARC, or wireless controller headphone jacks), avoiding issues where sound remains trapped in desktop PC speakers.

---

## 3. Game Launch Behaviors

Console Mode offers **two configurable execution behaviors**:

### Mode 1: Hide & Restore in Fullscreen (**Console Default**)
- **On Game Launch:** The launcher hides or minimizes its window, releasing 100% of GPU and window compositor overhead to the game.
- **During Gameplay:** The background process supervisor monitors the game or emulator process.
- **On Game Exit:** The launcher immediately restores in **fullscreen** and re-acquires controller focus instantly. The user never sees the desktop.

### Mode 2: Keep Open in Background
- Keeps the launcher interface running in the background while the game executes, ideal for multi-monitor setups where the user wishes to keep the library visible on a secondary screen.

---

## 4. 10-Foot Gamepad-First Navigation

- **Full Controller Support:** Native out-of-the-box compatibility with Xbox controllers (XInput), PlayStation DualShock 4 / DualSense, and Nintendo Switch Pro controllers.
- **Automatic Cursor Suppression:** As soon as any button on the gamepad is pressed, the mouse cursor is hidden from the display.
- **On-Screen Virtual Keyboard (OSK):** Allows searching games, typing titles, or entering Wi-Fi credentials using only the controller D-pad and buttons.
- **Console Audio Design:** Subtle, satisfying audio cues on navigation, focus, selection, and return actions.

---

## 5. Gamepad Power Menu & Wake-on-LAN

### 5.1 Controller Power Actions
By pressing the Guide/Home button (Xbox / PS) or selecting the power icon:
- **Sleep:** Enters low-power standby mode.
- **Hibernate:** Saves system state to disk and powers down.
- **Restart:** Full system reboot.
- **Shutdown:** Clean, safe OS shutdown.

### 5.2 Wake-on-LAN (WoL) for Moonlight
Helps configure network adapters so that Moonlight clients on TVs, handhelds, or smartphones can remotely power on the PC across the local network via Magic Packets.
