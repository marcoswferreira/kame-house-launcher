# Kame House Launcher 🐢🎮

> **Transform your PC into the ultimate living-room console experience.**  
> A high-performance game launcher built from scratch in **Rust**, featuring native **DualSenseX** adaptive trigger integration, Steam ecosystem bridges, and a dedicated **Console Mode** transformation suite.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Coverage](https://img.shields.io/badge/coverage-82.13%25-brightgreen.svg?style=flat-square)](#-testing-and-code-coverage)
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
  - [2. DualSense Adaptive Trigger Engine & DualSenseX UDP Server](#2-dualsense-adaptive-trigger-engine--dualsensex-udp-server)
  - [3. Dedicated Console Mode (Living Room PC)](#3-dedicated-console-mode-living-room-pc)
  - [4. Steam Ecosystem Interoperability](#4-steam-ecosystem-interoperability)
  - [5. Native Steam Achievement Relay](#5-native-steam-achievement-relay)
- [Prerequisites & Dependencies](#-prerequisites--dependencies)
- [Building & Running](#-building--running)
- [Testing & Code Coverage](#-testing-and-code-coverage)
- [Architectural Decision Records (ADRs)](#-architectural-decision-records-adrs)
- [Credits & Attributions](#-credits--attributions)
- [License](#-license)

---

## 🔥 Highlights

- ⚡ **100% Native Rust:** Ultra-low memory footprint (~40MB RAM compared to hundreds of MB consumed by traditional Electron-based launchers), maximizing available CPU and GPU resources for gaming and streaming.
- 📺 **Living Room Console Mode:** Painless password-free Auto-Login for Windows and Linux, silent fullscreen boot (Kiosk shell), TV resolution presets (4K @ 120Hz), HDR toggle, and automatic audio routing to HDMI/Soundbar.
- 🎮 **10-Foot Gamepad-First Navigation:** Full controller support (D-pad, thumbsticks, bumpers, triggers), tactile audio cues, automatic mouse cursor suppression, and an on-screen virtual keyboard (OSK).
- 🔄 **Configurable Launch Behaviors:** Choose between:
  1. **Hide/minimize on launch and return in fullscreen with immediate gamepad focus on exit** (Console Standard); OR
  2. **Keep the launcher running in the background**.
- 🎮 **Native DualSenseX Studio (100% Pure Rust):** Direct hardware control over Sony DualSense & Edge controllers. Adaptive triggers (Rigid Resistance, Weapon snap, Bow, Machine Gun, Galloping, Pulse vibration), RGB lightbar, player LEDs, and an async UDP server on port `6969` compatible with existing DualSenseX game mods (Assetto Corsa, F1, Cyberpunk 2077).
- 🏆 **Steam Achievement Relays:** Native toast notifications with official audio cues dispatched through the Steam CEF Chrome DevTools Protocol (CDP) for emulator and PC games.

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
├── docs/                           # Technical documentation and guides
│   ├── ARCHITECTURE.md             # System design and data flow diagrams
│   ├── CONSOLE_MODE.md             # Living room console transformation guide
│   └── adrs/                       # Architectural Decision Records
│       ├── 0001-rust-tauri-architecture.md
│       ├── 0003-game-scanner-and-rom-identification.md
│       ├── 0004-steam-injection-and-native-notifications.md
│       └── 0005-console-mode-transformation.md
└── crates/
    ├── launcher-core/              # Scanner, cartridge headers, disc parser, platforms
    ├── launcher-steam/             # Binary VDF shortcuts, grid artwork, CDP WebSocket bridge
    ├── launcher-achievements/      # Unified achievement tracker and emulator log parsers
    ├── launcher-dualsense/         # Pure Rust DualSense HID protocol, adaptive triggers, RGB, UDP 6969
    ├── launcher-console/           # TV display presets, audio routing, auto-login, launch supervisor
    └── launcher-app/               # Central coordinator service, React 19 UI, and kame-house-launcher CLI
```

---

## 🎮 Core Features

### 1. Content-Based Scanning & Header Inspection
Rather than relying on volatile or arbitrary filenames (like `game_final_v2.iso`), `launcher-core` inspects internal binary signatures:
- **Cartridge Headers:** Zero-allocation inspection for Game Boy Advance, Nintendo DS, Game Boy/Color, Nintendo 64 (supporting big-endian, byte-swapped, and word-swapped encodings), Sega Mega Drive/Genesis, and Super Nintendo (LoROM/HiROM checksum and complement verification).
- **Disc Sector Images:** Direct sector decoding of ISO9660 Primary Volume Descriptors (PVD), extracting `SYSTEM.CNF` for PS1/PS2 and parsing `PARAM.SFO` for PSP, PS3, and PS4.
- **PC Games & Stores:** Portable Executable (PE) metadata inspection (`ProductName`, `FileDescription`) and local Steam AppID discovery (`steam_appid.txt`, `steam_emu.ini`, Goldberg emulators, installation manifests).
- **Parallel Multi-Threading:** High-speed multi-disk traversal powered by `rayon` and `walkdir`.

### 2. DualSense Adaptive Trigger Engine & DualSenseX UDP Server
- **100% Pure Rust Implementation:** Re-architects DualSenseX capabilities natively in Rust. Zero C# or external runtime dependencies.
- **Full Adaptive Trigger Engine:** Low-level Sony HID reports (USB `0x02` and Bluetooth `0x31` with IEEE 802.3 CRC32 calculation) for Feedback, Semi-Auto Weapon breaks, Bow draw tension, Machine Gun kick, Galloping rhythm, and 10-zone custom resistance curves.
- **DualSenseX Link UDP Server (:6969):** Async UDP socket server accepting JSON instructions from existing telemetry mods (Assetto Corsa, F1 23/24, Cyberpunk 2077) without needing any extra software.
- **RGB Lightbar & Player LEDs:** Real-time halo lighting (Static, Breathing, Rainbow, Battery status) and Player 1-4 indicator dots.
- **Controller Emulation & Gyro Aiming:** Seamless XInput (Xbox 360) and DualShock 4 emulation via ViGEmBus/uinput, plus 6-axis IMU gyro-to-stick camera deflection.

### 3. Dedicated Console Mode (Living Room PC)
Engineered to remove desktop operating system friction:
- **Auto-Login Assistant:** Generates Windows `AutoAdminLogon` registry directives or Linux display manager configurations to bypass password prompts on startup.
- **Kiosk / Shell Mode:** Option to boot directly into the launcher in exclusive fullscreen, hiding taskbars, desktop icons, and OS notifications.
- **TV Display Switcher:** Instant presets for living room screens: 4K @ 120Hz, 1440p @ 120Hz, 1080p @ 120Hz, and one-click HDR enablement.
- **Audio Routing:** Automatically redirects default system audio to HDMI TV or Soundbar outputs.
- **Gamepad Power Actions:** Sleep, Hibernate, Restart, or Shutdown the PC directly using your controller.
- **Dual Launch Behaviors:**
  - `HideAndRestoreFullscreen` (Default): Hides or minimizes the launcher while playing, and immediately restores it in fullscreen with controller focus when the game exits.
  - `KeepOpen`: Keeps the launcher active in the background.

### 4. Steam Ecosystem Interoperability
- **Atomic Binary VDF Manipulation:** The `launcher-steam` crate parses and serializes Valve's binary format without corrupting existing shortcuts.
- **Deterministic Steam AppID Hashing:** Computes the 32-bit CRC32 shortcut identifier with bit 31 set:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Grid Artwork Management:** Places custom vertical posters (`p.png`), hero banners (`_hero.png`), transparent logos (`_logo.png`), wide capsules (`.png`), and executable icons (`_icon.ico`) into `userdata/<account>/config/grid`.

### 5. Native Steam Achievement Relay
- Connects to Steam's CEF debugging port (`127.0.0.1:8080`) over Chrome DevTools Protocol (CDP).
- When achievements are unlocked in emulators (RetroAchievements, RPCS3, Vita3K) or PC stores, an event is dispatched to Steam's `SharedJSContext`, triggering the official notification overlay with audio.

---

## 🛠 Prerequisites & Dependencies

- **Rust:** Version 1.75 or higher (`rustc` and `cargo`).
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

### 2. Build in Debug Mode
```bash
cargo build --workspace
```

### 3. Build in Release Mode (Optimized for Production)
```bash
cargo build --release --workspace
```
The optimized binary will be produced at `target/release/kame-house-launcher`.

### 4. Run the Launcher
```bash
cargo run -p launcher-app
```
Or directly via the compiled binary:
```bash
./target/release/kame-house-launcher
```

### 5. CLI & Internationalization (i18n)
The application defaults to English (`en`) and provides full support for Portuguese (`pt-BR`). You can switch languages temporarily per command or persist your preference permanently:

```bash
# Print status in English (default)
./target/release/kame-house-launcher status

# Switch preferred language to Brazilian Portuguese (persisted in settings.json)
./target/release/kame-house-launcher --set-lang pt-br

# Temporary run override in Portuguese without changing default config
./target/release/kame-house-launcher -l pt-br status

# Switch preferred language back to English
./target/release/kame-house-launcher --set-lang en

# View CLI commands & help in Portuguese
./target/release/kame-house-launcher -l pt-br --help
```

---

## 🧪 Testing and Code Coverage

The project enforces test-driven reliability and maintains over 80% code coverage across all crates:

### Run Full Test Suite
```bash
cargo test --workspace
```
All 34 unit and integration tests should pass:
```text
test result: ok. 34 passed; 0 failed; 0 ignored; finished in 0.25s
```

### Measure Code Coverage
Coverage is measured using LLVM source instrumentation via `cargo-llvm-cov`:
```bash
# Install tools if needed
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked

# Generate coverage summary
cargo llvm-cov --workspace --summary-only
```

**Current Verified Metrics:**
- **Line Coverage:** **82.13%**
- **Region Coverage:** **79.98%**
- **Function Coverage:** **80.69%**

---

## 📚 Architectural Decision Records (ADRs)

To understand the context and tradeoffs behind our architectural decisions, consult [`docs/adrs/`](docs/adrs/):
- [ADR 0001: Rust and Tauri v2 Architecture](docs/adrs/0001-rust-tauri-architecture.md)
- [ADR 0003: Content-Based Game Scanner and Metadata Engine](docs/adrs/0003-game-scanner-and-rom-identification.md)
- [ADR 0004: Steam Interoperability and Native Notifications](docs/adrs/0004-steam-injection-and-native-notifications.md)
- [ADR 0005: Living Room Console Mode Transformation](docs/adrs/0005-console-mode-transformation.md)

---

## 🤝 Contributing

Contributions are welcomed! Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for detailed guidelines on code conventions, PR workflows, and our **Conventional Commits** standard.

---

## 💖 Credits & Attributions

- **DualSenseX:** Pioneering open-source PlayStation DualSense controller customization and UDP link protocol created by [Paliverse](https://github.com/Paliverse/DualSenseX). Kame House Launcher implements this architecture in 100% native Rust.
- **Rust Ecosystem:** Sincere gratitude to the maintainers of `tokio`, `rayon`, `reqwest`, `serde`, `byteorder`, `walkdir`, and `crc32fast`.

---

## 📄 License

This project is free and open-source software licensed under the **GNU General Public License v3.0** (GPL-3.0-or-later). See the [LICENSE](LICENSE) file for the full text.
