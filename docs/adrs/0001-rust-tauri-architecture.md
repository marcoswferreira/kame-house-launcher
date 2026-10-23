# ADR 0001: Rust Workspace and Tauri v2 Architecture

- **Status:** Accepted
- **Date:** 2026-10-22
- **Deciders:** Engineering Team

<p align="center">
  <b>English</b> · <a href="0001-rust-tauri-architecture.pt-BR.md">Português (Brasil)</a>
</p>

## Context and Problem Statement

Kame House Launcher is conceived as a high-performance game launcher and central control system designed to transform everyday PCs into living-room gaming consoles.

The core technical requirements are:
1. A solid codebase in **Rust**, with strict modularity, strong typing, native performance, and memory safety.
2. High-performance low-level operations: multi-threaded traversal and hashing of thousands of ROMs/ISOs, parsing proprietary binary formats (Valve VDF, ISO9660, PARAM.SFO, PE executables), direct disc inspection, and OS-level system calls with a minimal background memory footprint (<60MB RAM).
3. A fluid 60/120fps 10-foot user interface with complete gamepad navigation, subtle tactile audio cues, and high-resolution artwork rendering.
4. Clean integration and embedding of the Sunshine streaming control plane and the "Console Mode" suite.

## Considered Options

1. **Option 1: Modular Rust Workspace + Tauri v2 (Recommended & Selected)**
   - 100% Rust backend distributed across specialized crates.
   - Native WebView frontend (WebKit/WebView2 via Tauri v2), leveraging modern web technologies (TypeScript/Tailwind) for rich 10-foot carousels, console TV layouts, and configuration dashboards.
   - Memory footprint of ~40MB to 60MB (dramatic reduction compared to heavyweight bundled browsers).
   - Native hardware acceleration and Gamepad API support.

2. **Option 2: Pure Rust GUI (Slint or Iced)**
   - Direct native rendering via Skia or wgpu.
   - Low memory usage (<30MB) and fast compilation.
   - *Downsides:* Lower flexibility for dynamic streaming previews/stats from Sunshine, higher implementation complexity for game carousels, animations, and ecosystem integrations.

## Decision Outcome

Chosen **Option 1**: A modular **Rust** Cargo workspace paired with **Tauri v2** for the presentation layer.

### Modular Crates Structure

```text
kame-house-launcher/
├── Cargo.toml (Root Workspace)
├── crates/
│   ├── launcher-core/         # Platform catalog, scanner, ROM headers, disc parser, PE inspection
│   ├── launcher-steam/        # shortcuts.vdf binary upsert, grid artwork, and CDP WebSocket bridge
│   ├── launcher-achievements/ # Unified achievement tracker (RetroAchievements, RPCS3, stores)
│   ├── launcher-sunshine/     # Sunshine REST HTTPS client, daemon runner, and apps.json sync
│   ├── launcher-console/      # Console Mode: auto-login, autostart, TV display, audio, supervisor
│   └── launcher-app/          # Central LauncherService, IPC commands, and kame-house-launcher entrypoint
```

## Consequences

### Positive
- **Efficiency and Performance:** Multi-disk scanning, cryptographic hashing, and VDF operations execute at native multi-threaded speeds (`rayon`/`tokio`).
- **Minimal Background Overhead:** The launcher can stay idle in the system tray or background consuming tens of megabytes while games or streaming sessions run.
- **Console-Grade TV Experience:** Modern WebViews enable PS5/Xbox-like smooth transitions, TV dark themes, and universal gamepad navigation.
- **Portability:** Cross-platform compilation for Windows and Linux.
