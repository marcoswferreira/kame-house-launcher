# Steam Companion 🎮

> **High-performance Steam Ecosystem Companion & DualSense Studio Extension.**  
> Built from scratch in **100% Native Rust**, Steam Companion connects non-Steam games and emulators into Valve's Steam ecosystem with atomic `shortcuts.vdf` synchronization, native Steam achievement notifications, System Tray lifecycle management, and an opt-in **DualSense Studio** hardware extension.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=flat-square)](https://conventionalcommits.org)

<p align="center">
  <b>English</b> · <a href="README.pt-BR.md">Português (Brasil)</a>
</p>

---

## 📋 Table of Contents
- [Highlights](#-highlights)
- [Workspace Architecture](#-workspace-architecture)
- [Core Features](#-core-features)
  - [1. Content-Based Scanning & Header Inspection](#1-content-based-scanning--header-inspection)
  - [2. Steam Ecosystem Interoperability (`shortcuts.vdf`)](#2-steam-ecosystem-interoperability-shortcutsvdf)
  - [3. Native Steam Achievement Relay](#3-native-steam-achievement-relay)
  - [4. DualSense Studio (Opt-In Hardware Extension)](#4-dualsense-studio-opt-in-hardware-extension)
  - [5. System Tray Lifecycle & On-Demand Debug Terminal](#5-system-tray-lifecycle--on-demand-debug-terminal)
- [Prerequisites & Dependencies](#-prerequisites--dependencies)
- [Building & Running](#-building--running)
- [Testing & Verification](#-testing--verification)
- [Architectural Decision Records (ADRs)](#-architectural-decision-records-adrs)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🔥 Highlights

- ⚡ **100% Native Rust:** Ultra-low memory footprint (~40MB RAM), maximizing available CPU and GPU resources for your games and emulators.
- 🔗 **Steam Library Bridge:** Automatically scans ROMs, ISOs, and standalone PC executables by inspecting internal binary headers and synchronizes them into Steam's binary `shortcuts.vdf` with deterministic 32-bit CRC32 AppIDs and Grid artwork.
- 🏆 **Native Steam Achievement Relay:** Dispatches official Steam toast notifications with audio cues through the Steam CEF Chrome DevTools Protocol (CDP) when achievements are unlocked in emulators or standalone titles.
- 🧩 **DualSense Studio (Opt-In Extension):** A modular hardware control plane for Sony DualSense & DualSense Edge controllers over USB and Bluetooth. Starts **disabled (`OFF`) by default** so it consumes zero background resources unless activated by the user or configured to autostart.
- 🖥️ **Clean Desktop & System Tray Experience:** Launches directly into the standalone frontend window without opening a background terminal. Closing the window (`X`) prompts to either **Minimize to the System Tray** or **Quit completely**, and a **Debug Terminal (`Logs`)** can be toggled on demand from the System Tray or UI header.

---

## 🏛 Workspace Architecture

The project is structured as a modular Cargo workspace, enforcing clean boundaries, testability, and component reusability:

```text
kame-house-launcher/
├── Cargo.toml                      # Root Cargo Workspace definition
├── Cargo.lock                      # Deterministic dependency lockfile
├── LICENSE                         # GNU General Public License v3.0
├── README.md                       # English documentation (default)
├── README.pt-BR.md                 # Portuguese documentation
├── CONTRIBUTING.md                 # Contribution and commit guidelines
├── frontend/                       # React 19 + TypeScript UI bundled into launcher-app
├── docs/                           # Technical documentation and guides
│   ├── ARCHITECTURE.md             # System design and data flow diagrams
│   └── adrs/                       # Architectural Decision Records
│       ├── 0001-rust-tauri-architecture.md
│       ├── 0003-game-scanner-and-rom-identification.md
│       └── 0004-steam-injection-and-native-notifications.md
└── crates/
    ├── launcher-core/              # Multi-threaded scanner, cartridge headers, disc parser, platforms
    ├── launcher-steam/             # Binary VDF shortcuts, grid artwork, CDP WebSocket bridge
    ├── launcher-achievements/      # Unified achievement tracker and emulator log parsers
    ├── launcher-dualsense/         # Pure Rust DualSense HID protocol, 19 trigger modes, RGB, UDP :6969
    ├── launcher-console/           # Audio device & microphone routing and launch supervisor
    └── launcher-app/               # Central coordinator service, HTTP server, System Tray & CLI
```

---

## 🎮 Core Features

### 1. Content-Based Scanning & Header Inspection
Rather than relying on arbitrary filenames, `launcher-core` inspects internal binary signatures:
- **Cartridge Headers:** Zero-allocation inspection for Game Boy Advance, Nintendo DS, Game Boy/Color, Nintendo 64 (supporting big-endian, byte-swapped, and word-swapped encodings), Sega Mega Drive/Genesis, and Super Nintendo (LoROM/HiROM checksum and complement verification).
- **Disc Sector Images:** Direct sector decoding of ISO9660 Primary Volume Descriptors (PVD), extracting `SYSTEM.CNF` for PS1/PS2 and parsing `PARAM.SFO` for PSP, PS3, and PS4.
- **PC Games & Stores:** Portable Executable (PE) metadata inspection (`ProductName`, `FileDescription`) and local Steam AppID discovery (`steam_appid.txt`, `steam_emu.ini`, installation manifests).
- **Parallel Multi-Threading:** High-speed multi-disk traversal powered by `rayon` and `walkdir`.

### 2. Steam Ecosystem Interoperability (`shortcuts.vdf`)
- **Atomic Binary VDF Manipulation:** The `launcher-steam` crate parses and serializes Valve's binary format without corrupting existing shortcuts.
- **Deterministic Steam AppID Hashing:** Computes the 32-bit CRC32 shortcut identifier with bit 31 set:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Grid Artwork Management:** Places custom vertical posters (`p.png`), hero banners (`_hero.png`), transparent logos (`_logo.png`), wide capsules (`.png`), and executable icons (`_icon.ico`) into `userdata/<account>/config/grid`.

### 3. Native Steam Achievement Relay
- Connects to Steam's CEF debugging port (`127.0.0.1:8080`) over Chrome DevTools Protocol (CDP).
- When achievements are unlocked in emulators (RPCS3, RetroArch, Vita3K) or standalone games, an event is dispatched to Steam's `SharedJSContext`, triggering the official notification overlay with audio.

### 4. DualSense Studio (Opt-In Hardware Extension)
- **Opt-In by Default:** The DualSense extension remains turned off by default so no background threads or USB/Bluetooth polling run unless requested. Users can enable it with one click in the UI and optionally check **"Start automatically with the application"**.
- **Real Hardware Telemetry:** Reports accurate USB/Bluetooth connection states and real battery percentage/charging status (showing `Disconnected` when no controller is plugged in).
- **19 Adaptive Trigger Modes:** Low-level Sony HID reports (USB `0x02` and Bluetooth `0x31` with IEEE 802.3 CRC32 calculation and Feature Report `0x05` unlock) supporting `Off`, `Feedback`, `Weapon`, `Bow`, `Galloping`, `MachineGun`, `Machine`, `Vibration`, `GameCube`, `Choppy`, `VerySoft`, `Soft`, `Medium`, `Hard`, `VeryHard`, `Hardest`, `Rigid`, `VibrateTriggerPulse`, and 10-zone `CustomZone` curves.
- **RGB Lightbar, Player LEDs & Mic Sync:** Real-time touchpad halo lighting (`Static`, `Breathing`, `Rainbow`, `Battery`), Player 1–4 indicator LEDs, and physical Microphone Mute button synchronization with OS audio mute.
- **Controller Tester, Emulation & UDP Server (:6969):** Built-in real-time input/stick tester, WebHID bridge, XInput/DS4 emulation profiles, 6-axis IMU gyro aiming, and an async UDP telemetry server on port `6969`.

### 5. System Tray Lifecycle & On-Demand Debug Terminal
- **Silent Window Startup:** Starts only the graphical frontend window, hiding the console window automatically on launch.
- **System Tray Integration:** Remains accessible in the Windows notification area (`System Tray`) with quick actions to reopen the frontend UI, toggle the **Debug Terminal (`Logs`)**, or quit the application.
- **Close Confirmation (`X`):** Clicking the close button asks whether you want to **Minimize to System Tray** (staying online in the background) or **Quit the Application** completely.

---

## 🛠 Prerequisites & Dependencies

- **Rust:** Version 1.75 or higher (`rustc` and `cargo`).
- **Node.js & npm:** Version 20+ (only required when rebuilding the `frontend/` bundle).
- **C/C++ Compiler & Build Tools:** `gcc`/`g++` and `pkg-config` (Linux) or MSVC Build Tools (Windows).
- **Supported Operating Systems:**
  - Windows 10/11 (64-bit).
  - Linux (Ubuntu 22.04+, Debian, Fedora, Arch Linux).

---

## 🚀 Building & Running

### 1. Clone Repository
```bash
git clone https://github.com/marcoswferreira/kame-house-launcher.git
cd kame-house-launcher
```

### 2. Build Frontend Assets (Optional — Pre-built assets are embedded)
```bash
npm install --prefix frontend
npm run build --prefix frontend
```

### 3. Build in Release Mode
```bash
cargo build --release --workspace
```
The optimized binary will be produced at `target/release/kame-house-launcher` (or `target\release\kame-house-launcher.exe` on Windows).

### 4. Run the Launcher
```bash
cargo run -p launcher-app
```

### 5. CLI & Internationalization (i18n)
The application defaults to English (`en`) and provides full support for Portuguese (`pt-BR`):

```bash
# Print status in English (default)
./target/release/kame-house-launcher status

# Switch preferred language to Brazilian Portuguese (persisted in settings.json)
./target/release/kame-house-launcher --set-lang pt-br

# Temporary run override in Portuguese
./target/release/kame-house-launcher -l pt-br status

# View CLI commands & help
./target/release/kame-house-launcher --help
```

---

## 🧪 Testing & Verification

### Run Full Test Suite
```bash
cargo test --workspace
```
All 53 unit and integration tests across the workspace pass:
```text
test result: ok. 53 passed; 0 failed; 0 ignored
```

### Run Linter & Formatting Checks
```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

---

## 📚 Architectural Decision Records (ADRs)

To understand the context and tradeoffs behind our architectural decisions, consult [`docs/adrs/`](docs/adrs/):
- [ADR 0001: Rust Workspace and Embedded UI Architecture](docs/adrs/0001-rust-tauri-architecture.md)
- [ADR 0003: Content-Based Game Scanner and Metadata Engine](docs/adrs/0003-game-scanner-and-rom-identification.md)
- [ADR 0004: Steam Interoperability and Native Notifications](docs/adrs/0004-steam-injection-and-native-notifications.md)

---

## 🤝 Contributing

Contributions are welcomed! Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for detailed guidelines on code conventions, PR workflows, and our **Conventional Commits** standard.

---

## 📄 License

This project is free and open-source software licensed under the **GNU General Public License v3.0** (GPL-3.0-or-later). See the [LICENSE](LICENSE) file for the full text.
