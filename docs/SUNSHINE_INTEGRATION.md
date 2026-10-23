# Headless Sunshine Streaming Integration 📡🎮

This document describes how **Kame House Launcher** extends [Sunshine](https://github.com/LizardByte/Sunshine) as a high-performance headless streaming service, replacing the default browser-based configuration with an integrated controller-first experience.

<p align="center">
  <b>English</b> · <a href="SUNSHINE_INTEGRATION.pt-BR.md">Português (Brasil)</a>
</p>

---

## 1. Integration Philosophy: Extension Without Appropriation

Sunshine is the premier open-source solution for ultra-low latency desktop capture and real-time hardware video encoding (NVENC, AMD AMF, Intel QuickSync, VAAPI, software). Kame House Launcher adheres to the following principles:
1. **Zero Modifications to Core C++ Engine:** All video/audio encoding, RTSP packetization, and input driver emulation remain 100% maintained by Sunshine's upstream code.
2. **Git Submodule & Upstream Tracking:** Sunshine is tracked as an official Git submodule pointing to the LizardByte repository, enabling clean, continuous updates.
3. **Frontend Replacement:** Sunshine's legacy web UI (which required opening desktop browsers and bypassing self-signed SSL warnings) is bypassed in favor of our native, controller-driven UI.

---

## 2. Local REST HTTPS API Communication

Sunshine exposes a local REST API on port `47990` (`confighttp.cpp`). The `launcher-sunshine` crate interacts with these endpoints directly:

### 2.1 Key Consumed Endpoints

| Endpoint | Method | Description | Role in Launcher |
| :--- | :---: | :--- | :--- |
| `/api/csrf-token` | `GET` | Retrieves the session CSRF token | Authentication for state-mutating requests |
| `/api/pin` | `POST` | Submits the 4-digit PIN entered by the user | Instant Moonlight client pairing on-screen |
| `/api/config` | `GET`/`POST` | Reads and applies streaming configuration | Adjusts encoders, bitrates, resolutions, ports |
| `/api/apps` | `GET`/`POST` | Manages streamed applications | Synchronizes scanned game library |
| `/api/apps/close` | `POST` | Terminates the active streaming app | Graceful session termination |
| `/api/restart` | `POST` | Restarts the streaming daemon | Hot-reloads configuration changes |
| `/api/logs` | `GET` | Retrieves recent daemon logs | Real-time connection diagnostics |

---

## 3. Moonlight Pairing Workflow

1. The user opens the **Moonlight** client on a living room TV, Steam Deck, or mobile device and selects "Add Host".
2. Moonlight displays a **4-digit PIN** on the remote screen.
3. Simultaneously, Kame House Launcher detects the incoming connection request or provides the **"Pair Moonlight Device"** button in the Sunshine tab.
4. The user types the 4-digit PIN directly into the launcher's dialog using their gamepad or virtual keyboard.
5. The `launcher-sunshine` crate sends the payload:
   ```json
   {
     "pin": "1234",
     "name": "Kame House Launcher"
   }
   ```
6. Sunshine validates the cryptographic handshake and approves the Moonlight client immediately—without ever opening a web browser.

---

## 4. Game Catalog Sync (`apps.json`)

The `launcher-sunshine` crate keeps Sunshine's `apps.json` synchronized with the launcher's game catalog:

```json
{
  "apps": [
    {
      "name": "Kame House Launcher (Console)",
      "cmd": "\"C:\\Program Files\\KameHouse\\kame-house-launcher.exe\" --fullscreen",
      "index": -1,
      "auto-detach": "true",
      "wait-all": "true",
      "exit-timeout": "5"
    },
    {
      "name": "Super Mario 64",
      "cmd": "\"C:\\Games\\sm64.z64\"",
      "index": -1,
      "image-path": "C:\\Program Files\\KameHouse\\art\\sm64_cover.png",
      "auto-detach": "true",
      "wait-all": "true",
      "exit-timeout": "5"
    }
  ]
}
```

This enables users streaming from their living room TV to choose:
- Launch directly into the full **Kame House Launcher Big Picture experience**; OR
- Launch individual games straight from Moonlight's grid.

---

## 5. Upstream Tracking & Updates

To keep Sunshine up to date with upstream improvements and security patches:

```bash
# Update the Sunshine submodule to the latest upstream release
git submodule update --remote --merge Sunshine

# Run integration tests
cargo test -p launcher-sunshine
```

### Credits & Licensing
Under the terms of the **GPL-3.0** license, all copyright for Sunshine belongs to **LizardByte & Sunshine Contributors**. Kame House Launcher claims no ownership over Sunshine's capture engine, extending it strictly as an autonomous headless daemon.
