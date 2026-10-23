# System Architecture: Kame House Launcher 🐢

This document outlines the system design, crate separation of concerns, data flows, and external integrations for **Kame House Launcher**.

<p align="center">
  <b>English</b> · <a href="ARCHITECTURE.pt-BR.md">Português (Brasil)</a>
</p>

---

## 1. Architectural Overview

Kame House Launcher is built as an event-driven, modular Rust application organized into specialized crates:

```mermaid
flowchart TD
    subgraph UI["Presentation Layer (10-Foot UI)"]
        Grid["Game Library (Grid & Carousel)"]
        SunshineTab["Sunshine Panel (PIN Pairing, Encoders)"]
        ConsoleTab["Console Mode (TV, Audio, Auto-Login)"]
    end

    subgraph Service["Coordination Layer (launcher-app)"]
        LauncherService["LauncherService (State Management & Lifecycle)"]
    end

    subgraph CoreCrates["Specialized Rust Crates"]
        Core["launcher-core (Scanner, Headers, Discs, Platforms)"]
        Steam["launcher-steam (VDF shortcuts, Grid art, CDP WebSocket)"]
        Achiev["launcher-achievements (Achievement Relay & Journal)"]
        Sun["launcher-sunshine (REST HTTPS Client & apps.json sync)"]
        Console["launcher-console (TV Display, Audio, Shell Kiosk, Supervisor)"]
    end

    subgraph External["External Subsystems"]
        SteamClient["Steam Client (shortcuts.vdf & CEF Port 8080)"]
        SunshineDaemon["Sunshine C++ Daemon (HTTPS 47990 / RTSP)"]
        MoonlightClient["Moonlight Client (TV / Steam Deck / Mobile)"]
        OS["Operating System (Win32 APIs / Linux xrandr / ALSA)"]
    end

    UI <--> LauncherService
    LauncherService --> Core
    LauncherService --> Steam
    LauncherService --> Achiev
    LauncherService --> Sun
    LauncherService --> Console

    Steam <-->|"WebSocket CDP"| SteamClient
    Achiev -->|"Notification Relay"| Steam
    Sun <-->|"REST HTTPS /api/*"| SunshineDaemon
    SunshineDaemon -->|"Video/Input"| MoonlightClient
    Console <-->|"System APIs"| OS
```

---

## 2. Crate Responsibilities

### `launcher-core`
- **Platform Catalog:** Immutable mapping of 30+ game systems, metadata IDs (IGDB, RetroAchievements, libretro), and supported extensions.
- **Cartridge Header Inspection (`headers.rs`):** Zero-allocation reading of ROM headers for Game Boy Advance, Nintendo DS, Game Boy/Color, Nintendo 64 (with endianness swapping), Sega Genesis, and Super Nintendo (LoROM/HiROM checksum complement verification).
- **Disc Sector Reader (`disc.rs`):** Low-level ISO9660 Primary Volume Descriptor (PVD) parser extracting `SYSTEM.CNF` and `PARAM.SFO`.
- **PE Metadata Inspection (`pe.rs`):** Extraction of `ProductName` and local Steam AppID discovery (`steam_appid.txt`, `steam_emu.ini`, manifest files).
- **Multi-threaded Scanner (`scanner.rs`):** Concurrent multi-disk traversal powered by `rayon` and `walkdir`.

### `launcher-steam`
- **Binary VDF Engine (`vdf.rs`):** Recursive parser and atomic serializer for Valve's binary Data Format (`shortcuts.vdf`).
- **Deterministic Shortcut AppID:** CRC32 32-bit hash calculation:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Grid Artwork Manager (`grid.rs`):** Positions covers, heroes, logos, wide banners, and icons into Steam's `grid/` folder.
- **CDP WebSocket Bridge (`cdp.rs`):** Async Chrome DevTools Protocol client connecting to Steam CEF on port `8080` to attach to `SharedJSContext` and trigger native achievement notifications.

### `launcher-achievements`
- **Unified Tracker (`manager.rs`):** Data models for achievements, unlock events, and persistent activity journal (`activity.json`).
- **Log Parsers (`models.rs`):** Detects achievement unlocks from emulator logs (such as RPCS3) and relays them to the Steam bridge.

### `launcher-sunshine`
- **REST HTTPS Client (`client.rs`):** Communicates with Sunshine on `https://127.0.0.1:47990`, handling CSRF tokens and self-signed local certificates.
- **PIN Pairing Flow:** Consumes `/api/pin` to handle Moonlight pairing prompts directly on-screen.
- **Catalog Synchronizer (`sync.rs`):** Maps scanned game titles and the full launcher into Sunshine's `apps.json`.
- **Upstream Tracking:** Sunshine maintained as an unmodified Git submodule in `Sunshine/`.

### `launcher-console`
- **Launch Supervisor (`supervisor.rs`):** Manages both execution behaviors:
  1. `HideAndRestoreFullscreen`: Minimizes/hides the launcher during gameplay and restores it in fullscreen with immediate gamepad focus on exit.
  2. `KeepOpen`: Keeps the launcher active in the background.
- **TV Display Settings (`models.rs`):** Presets for 4K @ 120Hz, 1440p @ 120Hz, 1080p @ 120Hz, and HDR activation.
- **System Boot (`boot.rs`):** Auto-login helpers (`AutoAdminLogon` on Windows / autostart desktop entry on Linux).
- **Power Management (`power.rs`):** Controller-mapped Sleep, Hibernate, Restart, and Shutdown commands.

### `launcher-app`
- **Central Coordinator (`service.rs`):** `LauncherService` aggregating state, game catalog persistence, settings, and IPC calls.
- **Internationalization Engine (`i18n.rs`):** Bilingual dictionary with English default (`en`) and complete Portuguese (`pt-BR`) support.
- **CLI Commands & Flags (`cli.rs`):** Command-line parser supporting `--lang`, `--set-lang`, `status`, `scan`, `export-steam`, `sync-sunshine`, and `set-mode`.
- **Application Binary (`main.rs`):** CLI entrypoint with structured logging via `tracing-subscriber`.
