use crate::LauncherService;
use launcher_core::models::{Language, LaunchBehavior};
use launcher_dualsense::{DualTriggerConfig, EmulationConfig, LightingConfig};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{error, info};

// Embedded UI Assets
const INDEX_HTML: &str = include_str!("../ui/index.html");
const STYLE_CSS: &str = include_str!("../ui/style.css");
const APP_JS: &str = include_str!("../ui/app.js");
const LOGO_SVG: &str = include_str!("../../../assets/branding/logo.svg");
const HERO_JPG: &[u8] = include_bytes!("../../../assets/branding/kame-house-hero.jpg");

#[derive(Serialize)]
struct StatusResponse {
    version: &'static str,
    language: String,
    games_count: usize,
    launch_behavior: String,
    dualsense_extension_enabled: bool,
    dualsense_autostart: bool,
    dualsense_connected: bool,
    dualsense_udp_enabled: bool,
    scan_directories: Vec<String>,
    debug_console_visible: bool,
}

#[derive(Deserialize)]
struct LaunchRequest {
    id: String,
}

#[derive(Deserialize)]
struct ScanRequest {
    paths: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct TriggerProfileRequest {
    id: String,
}

#[derive(Deserialize)]
struct ToggleUdpRequest {
    enabled: bool,
}

#[derive(Deserialize)]
struct ToggleExtensionRequest {
    enabled: Option<bool>,
    autostart: Option<bool>,
}

#[derive(Deserialize)]
struct SettingsUpdateRequest {
    language: Option<String>,
    launch_behavior: Option<String>,
}

#[derive(Deserialize)]
struct OpenUrlRequest {
    url: String,
}

pub struct HttpServer {
    service: Arc<LauncherService>,
    addr: SocketAddr,
}

impl HttpServer {
    pub fn new(service: Arc<LauncherService>, port: u16) -> Self {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        Self { service, addr }
    }

    pub async fn bind_with_fallback(
        service: Arc<LauncherService>,
        preferred_port: u16,
    ) -> std::io::Result<(Self, TcpListener, u16)> {
        let mut ports = vec![preferred_port];
        for p in 48080..=48095 {
            if !ports.contains(&p) {
                ports.push(p);
            }
        }
        for p in [8080, 8081, 3000, 5000, 0] {
            if !ports.contains(&p) {
                ports.push(p);
            }
        }

        let mut last_err = None;
        for port in ports {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            match TcpListener::bind(addr).await {
                Ok(listener) => {
                    let local_addr = listener.local_addr()?;
                    let actual_port = local_addr.port();
                    if actual_port != preferred_port {
                        tracing::warn!(
                            "Preferred port {} was unavailable. Kame House Big Picture Server bound to fallback port {}",
                            preferred_port,
                            actual_port
                        );
                    }
                    return Ok((
                        HttpServer {
                            service,
                            addr: local_addr,
                        },
                        listener,
                        actual_port,
                    ));
                }
                Err(e) => {
                    tracing::debug!("Port {} unavailable: {}", port, e);
                    last_err = Some(e);
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Could not bind to any port",
            )
        }))
    }

    pub async fn serve(self, listener: TcpListener) -> std::io::Result<()> {
        info!(
            "Kame House Big Picture Server listening on http://{}",
            self.addr
        );

        loop {
            let (stream, _) = listener.accept().await?;
            let svc = Arc::clone(&self.service);
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, svc).await {
                    error!("HTTP Connection error: {}", e);
                }
            });
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        self.serve(listener).await
    }
}

pub async fn handle_connection<S>(
    mut stream: S,
    service: Arc<LauncherService>,
) -> std::io::Result<()>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    let mut buffer = [0u8; 8192];
    let bytes_read = stream.read(&mut buffer).await?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request_str.lines();
    let first_line = lines.next().unwrap_or("");
    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    let (status, content_type, body) = match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            service.lifecycle.record_page_activity();
            (
                "200 OK",
                "text/html; charset=utf-8",
                INDEX_HTML.as_bytes().to_vec(),
            )
        }
        ("GET", "/style.css") => (
            "200 OK",
            "text/css; charset=utf-8",
            STYLE_CSS.as_bytes().to_vec(),
        ),
        ("GET", "/app.js") => {
            service.lifecycle.record_page_activity();
            (
                "200 OK",
                "application/javascript; charset=utf-8",
                APP_JS.as_bytes().to_vec(),
            )
        }
        ("GET", "/logo.svg") => ("200 OK", "image/svg+xml", LOGO_SVG.as_bytes().to_vec()),
        ("GET", "/kame-house-hero.jpg") => ("200 OK", "image/jpeg", HERO_JPG.to_vec()),
        ("GET", "/api/status") => {
            service.lifecycle.record_page_activity();
            let games = service.list_games();
            let settings = service.settings.read().clone();
            let behavior_str = match settings.launch_behavior {
                LaunchBehavior::HideAndRestoreFullscreen => "hide_and_restore_fullscreen",
                LaunchBehavior::KeepOpen => "keep_open",
            };
            let ds_status = service.dualsense.get_status().await;
            let resp = StatusResponse {
                version: env!("CARGO_PKG_VERSION"),
                language: settings.language.code().to_string(),
                games_count: games.len(),
                launch_behavior: behavior_str.to_string(),
                dualsense_extension_enabled: ds_status.extension_enabled,
                dualsense_autostart: ds_status.autostart_on_launch,
                dualsense_connected: ds_status.connected,
                dualsense_udp_enabled: settings.dualsense_udp_enabled,
                scan_directories: settings.scan_directories.clone(),
                debug_console_visible: service.lifecycle.is_debug_console_visible(),
            };
            let json = serde_json::to_vec(&resp).unwrap_or_default();
            ("200 OK", "application/json", json)
        }
        ("GET", "/api/games") => {
            let games = service.list_games();
            let json = serde_json::to_vec(&games).unwrap_or_default();
            ("200 OK", "application/json", json)
        }
        ("POST", "/api/launch") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<LaunchRequest>(&body_str) {
                if let Some((game, state_before)) = service.prepare_launch(&req.id) {
                    info!(
                        "Launching game '{}' with state {:?}",
                        game.title, state_before
                    );
                    (
                        "200 OK",
                        "application/json",
                        b"{\"status\":\"launched\"}".to_vec(),
                    )
                } else {
                    (
                        "404 Not Found",
                        "application/json",
                        b"{\"error\":\"game not found\"}".to_vec(),
                    )
                }
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid body\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/settings") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<SettingsUpdateRequest>(&body_str) {
                let mut settings = service.settings.read().clone();
                if let Some(l) = req.language {
                    if let Some(parsed) = Language::parse_flexible(&l) {
                        settings.language = parsed;
                    }
                }
                if let Some(b) = req.launch_behavior {
                    if b == "keep_open" {
                        settings.launch_behavior = LaunchBehavior::KeepOpen;
                    } else if b == "hide_and_restore_fullscreen" {
                        settings.launch_behavior = LaunchBehavior::HideAndRestoreFullscreen;
                    }
                }
                service.update_console_settings(settings);
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"updated\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid json\"}".to_vec(),
                )
            }
        }
        ("GET", "/api/dualsense/status") => {
            let status = service.dualsense.get_status().await;
            let json = serde_json::to_vec(&status).unwrap_or_default();
            ("200 OK", "application/json", json)
        }
        ("POST", "/api/dualsense/trigger") => {
            let body_str = extract_body(&request_str);
            if let Ok(triggers) = serde_json::from_str::<DualTriggerConfig>(&body_str) {
                service.dualsense.set_triggers(triggers);
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"applied\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid trigger config\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/lightbar") => {
            let body_str = extract_body(&request_str);
            if let Ok(lighting) = serde_json::from_str::<LightingConfig>(&body_str) {
                service.dualsense.set_lighting(lighting);
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"applied\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid lighting config\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/emulation") => {
            let body_str = extract_body(&request_str);
            if let Ok(cfg) = serde_json::from_str::<EmulationConfig>(&body_str) {
                service.dualsense.set_emulation(cfg);
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"applied\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid emulation config\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/profile") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<TriggerProfileRequest>(&body_str) {
                if service.dualsense.apply_profile(&req.id) {
                    (
                        "200 OK",
                        "application/json",
                        b"{\"status\":\"profile_applied\"}".to_vec(),
                    )
                } else {
                    (
                        "404 Not Found",
                        "application/json",
                        b"{\"error\":\"unknown profile id\"}".to_vec(),
                    )
                }
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid profile request\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/extension/toggle") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<ToggleExtensionRequest>(&body_str) {
                if let Some(autostart) = req.autostart {
                    service.set_dualsense_autostart(autostart);
                }
                if let Some(enabled) = req.enabled {
                    service.set_dualsense_extension_active(enabled);
                    if !enabled {
                        service.dualsense.stop_udp().await;
                    } else if service.settings.read().dualsense_udp_enabled {
                        let _ = service.dualsense.start_udp().await;
                    }
                }
                let resp = format!(
                    "{{\"extension_enabled\":{},\"autostart_on_launch\":{}}}",
                    service.dualsense.is_extension_enabled(),
                    service.dualsense.is_autostart_enabled()
                );
                ("200 OK", "application/json", resp.into_bytes())
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid extension toggle request\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/udp/toggle") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<ToggleUdpRequest>(&body_str) {
                let mut settings = service.settings.read().clone();
                settings.dualsense_udp_enabled = req.enabled;
                service.update_console_settings(settings);
                if req.enabled {
                    let _ = service.dualsense.start_udp().await;
                } else {
                    service.dualsense.stop_udp().await;
                }
                (
                    "200 OK",
                    "application/json",
                    format!("{{\"udp_enabled\":{}}}", req.enabled).into_bytes(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid toggle request\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/dualsense/mic/toggle") => {
            let is_muted = service.dualsense.toggle_microphone();
            let _ = launcher_console::execute_set_microphone_mute(is_muted);
            let resp = format!("{{\"mic_muted\":{}}}", is_muted);
            ("200 OK", "application/json", resp.into_bytes())
        }
        ("POST", "/api/steam/sync") => {
            let vdf_path = dirs::data_dir()
                .map(|p| p.join("Steam/userdata"))
                .and_then(|p| {
                    if p.is_dir() {
                        if let Ok(entries) = std::fs::read_dir(&p) {
                            for entry in entries.flatten() {
                                let sub = entry.path().join("config/shortcuts.vdf");
                                if entry.path().is_dir() {
                                    return Some(sub);
                                }
                            }
                        }
                    }
                    None
                })
                .unwrap_or_else(|| PathBuf::from("./shortcuts.vdf"));

            match service.export_games_to_steam(&vdf_path) {
                Ok(count) => {
                    let resp = format!(
                        "{{\"status\":\"synchronized\",\"count\":{},\"path\":\"{}\"}}",
                        count,
                        vdf_path.to_string_lossy().replace('\\', "\\\\")
                    );
                    ("200 OK", "application/json", resp.into_bytes())
                }
                Err(e) => {
                    let resp = format!("{{\"error\":\"{}\"}}", e);
                    ("500 Internal Error", "application/json", resp.into_bytes())
                }
            }
        }
        ("POST", "/api/scan") => {
            let body_str = extract_body(&request_str);
            let mut settings = service.settings.read().clone();
            let roots: Vec<PathBuf> =
                if let Ok(req) = serde_json::from_str::<ScanRequest>(&body_str) {
                    if let Some(user_paths) = req.paths {
                        if !user_paths.is_empty() {
                            settings.scan_directories = user_paths.clone();
                            service.update_console_settings(settings);
                            user_paths.into_iter().map(PathBuf::from).collect()
                        } else if !settings.scan_directories.is_empty() {
                            settings
                                .scan_directories
                                .into_iter()
                                .map(PathBuf::from)
                                .collect()
                        } else {
                            vec![dirs::home_dir()
                                .map(|h| h.join("Games"))
                                .unwrap_or_else(|| PathBuf::from("./games"))]
                        }
                    } else if !settings.scan_directories.is_empty() {
                        settings
                            .scan_directories
                            .into_iter()
                            .map(PathBuf::from)
                            .collect()
                    } else {
                        vec![dirs::home_dir()
                            .map(|h| h.join("Games"))
                            .unwrap_or_else(|| PathBuf::from("./games"))]
                    }
                } else if !settings.scan_directories.is_empty() {
                    settings
                        .scan_directories
                        .into_iter()
                        .map(PathBuf::from)
                        .collect()
                } else {
                    vec![dirs::home_dir()
                        .map(|h| h.join("Games"))
                        .unwrap_or_else(|| PathBuf::from("./games"))]
                };

            let found = service.scan_and_add(&roots, 4);
            let paths_json = serde_json::to_string(
                &roots
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect::<Vec<_>>(),
            )
            .unwrap_or_else(|_| "[]".to_string());
            let resp = format!(
                "{{\"status\":\"scanned\",\"added\":{},\"paths\":{}}}",
                found, paths_json
            );
            ("200 OK", "application/json", resp.into_bytes())
        }
        ("GET", "/api/scan/paths") => {
            let settings = service.settings.read().clone();
            let json = serde_json::to_vec(&settings.scan_directories).unwrap_or_default();
            ("200 OK", "application/json", json)
        }
        ("POST", "/api/scan/paths") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<ScanRequest>(&body_str) {
                if let Some(user_paths) = req.paths {
                    let mut settings = service.settings.read().clone();
                    settings.scan_directories = user_paths;
                    service.update_console_settings(settings);
                    (
                        "200 OK",
                        "application/json",
                        b"{\"status\":\"paths_saved\"}".to_vec(),
                    )
                } else {
                    (
                        "400 Bad Request",
                        "application/json",
                        b"{\"error\":\"missing paths\"}".to_vec(),
                    )
                }
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid json\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/open-url") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<OpenUrlRequest>(&body_str) {
                let url = req.url.trim();
                if url.starts_with("http://") || url.starts_with("https://") {
                    open_system_browser(url);
                    (
                        "200 OK",
                        "application/json",
                        b"{\"status\":\"opened\"}".to_vec(),
                    )
                } else {
                    (
                        "400 Bad Request",
                        "application/json",
                        b"{\"error\":\"invalid protocol, must be http or https\"}".to_vec(),
                    )
                }
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid json\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/window/toggle-console") => {
            let visible = crate::tray::toggle_debug_console(&service.lifecycle);
            let resp = format!("{{\"debug_console_visible\":{}}}", visible);
            ("200 OK", "application/json", resp.into_bytes())
        }
        ("POST", "/api/window/minimize") => {
            service.lifecycle.record_explicit_action();
            crate::tray::notify_minimized_to_tray(service.language());
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"minimized_to_tray\"}".to_vec(),
            )
        }
        ("POST", "/api/window/quit") => {
            service.lifecycle.record_explicit_action();
            info!("User requested application quit via Frontend UI");
            crate::tray::cleanup_system_tray();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                std::process::exit(0);
            });
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"quitting\"}".to_vec(),
            )
        }
        ("POST", "/api/window/native-close") => {
            let close_ts = crate::tray::current_time_ms();
            let svc = Arc::clone(&service);
            tokio::spawn(async move {
                // Wait 450ms to distinguish a real window close from a page refresh (F5)
                tokio::time::sleep(std::time::Duration::from_millis(450)).await;
                if svc.lifecycle.was_recently_active_or_explicit(close_ts) {
                    return;
                }
                let lang = svc.language();
                let decision = tokio::task::spawn_blocking(move || {
                    crate::tray::prompt_native_close_decision(lang)
                })
                .await
                .unwrap_or(crate::tray::CloseDecision::MinimizeToTray);

                match decision {
                    crate::tray::CloseDecision::MinimizeToTray => {
                        info!("Native close prompt: User chose Minimize to System Tray");
                        crate::tray::notify_minimized_to_tray(svc.language());
                    }
                    crate::tray::CloseDecision::QuitApplication => {
                        info!("Native close prompt: User chose Quit Application");
                        crate::tray::cleanup_system_tray();
                        std::process::exit(0);
                    }
                    crate::tray::CloseDecision::ReopenWindow => {
                        info!("Native close prompt: User chose Cancel (Reopen Frontend Window)");
                        let url = svc.lifecycle.get_ui_url();
                        open_fullscreen_ui(&url);
                    }
                }
            });
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"close_handled\"}".to_vec(),
            )
        }
        _ => ("404 Not Found", "text/plain", b"404 Not Found".to_vec()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        status,
        content_type,
        body.len()
    );

    stream.write_all(response.as_bytes()).await?;
    stream.write_all(&body).await?;
    stream.flush().await?;

    Ok(())
}

fn extract_body(request: &str) -> String {
    if let Some(pos) = request.find("\r\n\r\n") {
        request[pos + 4..].to_string()
    } else if let Some(pos) = request.find("\n\n") {
        request[pos + 2..].to_string()
    } else {
        String::new()
    }
}

/// Opens a standard web URL in the user's default external browser.
pub fn open_system_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/c", "start", "", url])
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::fs::read_to_string("/proc/version")
            .map(|v| {
                let lower = v.to_lowercase();
                lower.contains("microsoft") || lower.contains("wsl")
            })
            .unwrap_or(false);

        if is_wsl {
            if Command::new("cmd.exe")
                .args(["/c", "start", url])
                .spawn()
                .is_err()
                && Command::new("wslview").arg(url).spawn().is_err()
            {
                tracing::info!("WSL detected. Please open URL in host: {}", url);
            }
            return;
        }

        let _ = Command::new("xdg-open").arg(url).spawn();
    }
}

/// Spawns the system browser in App Mode to display the frontend UI without opening a terminal window.
pub fn open_fullscreen_ui(url: &str) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        // Launch Microsoft Edge in standalone App Mode without spawning a visible cmd.exe window
        if Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/c",
                "start",
                "",
                "msedge",
                &format!("--app={}", url),
                "--window-size=1360,840",
            ])
            .spawn()
            .is_err()
        {
            // Fallback to default browser without console window
            let _ = Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/c", "start", "", url])
                .spawn();
        }
    }

    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::fs::read_to_string("/proc/version")
            .map(|v| {
                let lower = v.to_lowercase();
                lower.contains("microsoft") || lower.contains("wsl")
            })
            .unwrap_or(false);

        if is_wsl {
            // Under WSL, open via Windows host interop
            if Command::new("cmd.exe")
                .args(["/c", "start", url])
                .spawn()
                .is_err()
                && Command::new("powershell.exe")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!("Start-Process '{}'", url),
                    ])
                    .spawn()
                    .is_err()
                && Command::new("wslview").arg(url).spawn().is_err()
            {
                tracing::info!("WSL detected. Please open {} in your Windows browser.", url);
            }
            return;
        }

        let has_display =
            std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();
        if !has_display {
            tracing::info!(
                "Headless mode (no $DISPLAY). Please open {} in your browser.",
                url
            );
            return;
        }

        // Try Chrome / Chromium in App Mode
        if Command::new("google-chrome")
            .args([&format!("--app={}", url), "--start-fullscreen"])
            .spawn()
            .is_err()
            && Command::new("chromium")
                .args([&format!("--app={}", url), "--start-fullscreen"])
                .spawn()
                .is_err()
        {
            // Fallback to xdg-open
            let _ = Command::new("xdg-open").arg(url).spawn();
        }
    }
}
