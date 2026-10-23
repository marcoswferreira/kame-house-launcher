# ADR 0004: Steam Interoperability, VDF Binary Injection, and Native Notifications

- **Status:** Accepted
- **Date:** 2026-10-22
- **Deciders:** Engineering Team

<p align="center">
  <b>English</b> · <a href="0004-steam-injection-and-native-notifications.pt-BR.md">Português (Brasil)</a>
</p>

## Context and Problem Statement

To seamlessly integrate emulator titles and PC storefront games into the native Steam ecosystem in a living room setup:
1. Shortcuts must be written to `shortcuts.vdf` without corrupting Valve's binary format or existing shortcuts.
2. Custom artwork must be placed into `userdata/<account>/config/grid`.
3. Achievement unlocks from emulators (RetroAchievements, RPCS3, Vita3K) and PC stores must trigger Steam's official native achievement toast overlay with sound and animations.

This infrastructure must be implemented in pure Rust with maximum reliability, without requiring heavyweight third-party frameworks like SFP or Millennium.

## Decision Outcome

Implement this functionality across two specialized crates:
- `launcher-steam`: Valve binary VDF parser/encoder and Chrome DevTools Protocol (CDP) WebSocket injector.
- `launcher-achievements`: Unified achievement tracker and notification relay.

### 1. Pure Rust Binary VDF Engine (`shortcuts.vdf`)
- Recursive parser for binary Valve Data Format (`0x00` object, `0x01` string, `0x02` int32, `0x03` float32, `0x07` int64, `0x08` end marker).
- Deterministic 32-bit CRC32 Shortcut AppID generation:
  $$\text{shortcut\_id} = (\text{CRC32}(\text{exe} + \text{AppName}) \mid 0x80000000)$$
- Atomic file writes (`tempfile` + `rename`) to protect against sudden power loss.
- Artwork placement into Steam's grid folder:
  - Vertical cover: `<shortcut_id>p.png`
  - Hero banner: `<shortcut_id>_hero.png`
  - Logo: `<shortcut_id>_logo.png`
  - Wide capsule: `<shortcut_id>.png`
  - Icon: `<shortcut_id>_icon.ico`

### 2. Chrome DevTools Protocol (CDP) WebSocket Bridge
- Steam is launched with `-cef-enable-debugging -devtools-port 8080`.
- The launcher connects over WebSocket to `ws://127.0.0.1:8080/devtools/browser/...` using `tokio-tungstenite`.
- Attaches to `SharedJSContext` to dispatch native achievement toasts:
  ```javascript
  window.SteamClient.Notifications.ShowCustomNotification(title, description, iconUrl, appId);
  ```

## Consequences

### Positive
- **Complete Immersion:** Playing a PS3 title via RPCS3 or a SNES classic displays the official Steam achievement notification overlay in the corner of your TV screen with authentic audio.
- **Safety & Stability:** Uses official Chromium debugging APIs rather than brittle DLL or memory injection.
- **Big Picture Compatibility:** Operates smoothly in both standard desktop Steam and Steam Big Picture Mode.
