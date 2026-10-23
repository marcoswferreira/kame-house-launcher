use crate::LauncherService;
use launcher_console::power::get_power_command;
use launcher_console::PowerAction;
use launcher_core::models::{Language, LaunchBehavior};
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
    sunshine_url: String,
    sunshine_enabled: bool,
}

#[derive(Deserialize)]
struct LaunchRequest {
    id: String,
}

#[derive(Deserialize)]
struct PinRequest {
    pin: String,
}

#[derive(Deserialize)]
struct ToggleSunshineRequest {
    enabled: bool,
}

#[derive(Deserialize)]
struct UnpairRequest {
    uuid: String,
}

#[derive(Deserialize)]
struct SettingsUpdateRequest {
    language: Option<String>,
    launch_behavior: Option<String>,
}

#[derive(Deserialize)]
struct PowerRequest {
    action: String,
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

    pub async fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
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
        ("GET", "/") | ("GET", "/index.html") => (
            "200 OK",
            "text/html; charset=utf-8",
            INDEX_HTML.as_bytes().to_vec(),
        ),
        ("GET", "/style.css") => (
            "200 OK",
            "text/css; charset=utf-8",
            STYLE_CSS.as_bytes().to_vec(),
        ),
        ("GET", "/app.js") => (
            "200 OK",
            "application/javascript; charset=utf-8",
            APP_JS.as_bytes().to_vec(),
        ),
        ("GET", "/logo.svg") => ("200 OK", "image/svg+xml", LOGO_SVG.as_bytes().to_vec()),
        ("GET", "/kame-house-hero.jpg") => ("200 OK", "image/jpeg", HERO_JPG.to_vec()),
        ("GET", "/api/status") => {
            let games = service.list_games();
            let settings = service.settings.read().clone();
            let behavior_str = match settings.launch_behavior {
                LaunchBehavior::HideAndRestoreFullscreen => "hide_and_restore_fullscreen",
                LaunchBehavior::KeepOpen => "keep_open",
            };
            let resp = StatusResponse {
                version: env!("CARGO_PKG_VERSION"),
                language: settings.language.code().to_string(),
                games_count: games.len(),
                launch_behavior: behavior_str.to_string(),
                sunshine_url: service.sunshine.base_url.clone(),
                sunshine_enabled: settings.sunshine_enabled,
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
        ("POST", "/api/sunshine/toggle") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<ToggleSunshineRequest>(&body_str) {
                let mut settings = service.settings.read().clone();
                settings.sunshine_enabled = req.enabled;
                service.update_console_settings(settings);
                (
                    "200 OK",
                    "application/json",
                    format!("{{\"sunshine_enabled\":{}}}", req.enabled).into_bytes(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid json\"}".to_vec(),
                )
            }
        }
        ("GET", "/api/sunshine/config") => match service.sunshine.get_config().await {
            Ok(cfg) => (
                "200 OK",
                "application/json",
                serde_json::to_vec(&cfg).unwrap_or_default(),
            ),
            Err(_) => (
                "200 OK",
                "application/json",
                b"{\"sunshine_name\":\"Kame House\",\"min_log_level\":2,\"port\":47989}".to_vec(),
            ),
        },
        ("POST", "/api/sunshine/config") => {
            let body_str = extract_body(&request_str);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&body_str) {
                let _ = service.sunshine.update_config(val).await;
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
        ("GET", "/api/sunshine/clients") => match service.sunshine.get_clients().await {
            Ok(clients) => (
                "200 OK",
                "application/json",
                serde_json::to_vec(&clients).unwrap_or_default(),
            ),
            Err(_) => ("200 OK", "application/json", b"[]".to_vec()),
        },
        ("POST", "/api/sunshine/clients/unpair") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<UnpairRequest>(&body_str) {
                let _ = service.sunshine.unpair_client(&req.uuid).await;
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"unpaired\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid uuid\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/sunshine/clients/unpair-all") => {
            let _ = service.sunshine.unpair_all_clients().await;
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"unpaired_all\"}".to_vec(),
            )
        }
        ("GET", "/api/sunshine/apps") => {
            match service.sunshine.get_apps().await {
                Ok(apps) => (
                    "200 OK",
                    "application/json",
                    serde_json::to_vec(&apps).unwrap_or_default(),
                ),
                Err(_) => {
                    // Fallback to local apps.json if available
                    let path = PathBuf::from("./apps.json");
                    if path.is_file() {
                        let bytes = std::fs::read(path).unwrap_or_default();
                        ("200 OK", "application/json", bytes)
                    } else {
                        ("200 OK", "application/json", b"{\"apps\":[]}".to_vec())
                    }
                }
            }
        }
        ("POST", "/api/sunshine/apps") => (
            "200 OK",
            "application/json",
            b"{\"status\":\"saved\"}".to_vec(),
        ),
        ("GET", "/api/sunshine/logs") => match service.sunshine.get_logs().await {
            Ok(logs) => ("200 OK", "text/plain; charset=utf-8", logs.into_bytes()),
            Err(_) => (
                "200 OK",
                "text/plain; charset=utf-8",
                b"[KAME-HOUSE] Sunshine daemon standby (waiting for client connection)".to_vec(),
            ),
        },
        ("POST", "/api/sunshine/restart") => {
            let _ = service.sunshine.restart_service().await;
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"restarting\"}".to_vec(),
            )
        }
        ("POST", "/api/sunshine/reset-display-device-persistence") => {
            let _ = service.sunshine.reset_display_persistence().await;
            (
                "200 OK",
                "application/json",
                b"{\"status\":\"reset\"}".to_vec(),
            )
        }
        ("POST", "/api/sunshine/pin") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<PinRequest>(&body_str) {
                match service.sunshine.pair_pin(&req.pin, None).await {
                    Ok(_) => (
                        "200 OK",
                        "application/json",
                        b"{\"status\":\"paired\"}".to_vec(),
                    ),
                    Err(_) => (
                        "502 Bad Gateway",
                        "application/json",
                        b"{\"error\":\"sunshine unreachable or invalid pin\"}".to_vec(),
                    ),
                }
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid pin\"}".to_vec(),
                )
            }
        }
        ("POST", "/api/sunshine/sync") => {
            let path = PathBuf::from("./apps.json");
            let exe = env::current_exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "kame-house-launcher".to_string());
            match service.sync_sunshine_catalog(&path, &exe) {
                Ok(count) => {
                    let resp = format!("{{\"status\":\"synchronized\",\"count\":{}}}", count);
                    ("200 OK", "application/json", resp.into_bytes())
                }
                Err(e) => {
                    let resp = format!("{{\"error\":\"{}\"}}", e);
                    ("500 Internal Error", "application/json", resp.into_bytes())
                }
            }
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
            let roots = vec![dirs::home_dir()
                .map(|h| h.join("Games"))
                .unwrap_or_else(|| PathBuf::from("./games"))];
            let found = service.scan_and_add(&roots, 4);
            let resp = format!("{{\"status\":\"scanned\",\"added\":{}}}", found);
            ("200 OK", "application/json", resp.into_bytes())
        }
        ("POST", "/api/power") => {
            let body_str = extract_body(&request_str);
            if let Ok(req) = serde_json::from_str::<PowerRequest>(&body_str) {
                let action = match req.action.as_str() {
                    "sleep" => PowerAction::Sleep,
                    "restart" => PowerAction::Restart,
                    "shutdown" => PowerAction::Shutdown,
                    _ => PowerAction::Sleep,
                };
                let (cmd, args) = get_power_command(action);
                let _ = Command::new(cmd).args(args).spawn();
                (
                    "200 OK",
                    "application/json",
                    b"{\"status\":\"power_dispatched\"}".to_vec(),
                )
            } else {
                (
                    "400 Bad Request",
                    "application/json",
                    b"{\"error\":\"invalid action\"}".to_vec(),
                )
            }
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

/// Spawns the system browser in full-screen/kiosk mode to display the 10-foot UI.
pub fn open_fullscreen_ui(url: &str) {
    #[cfg(target_os = "windows")]
    {
        // Try Microsoft Edge in App Mode (borderless, full screen)
        if Command::new("cmd")
            .args([
                "/c",
                "start",
                "msedge",
                &format!("--app={}", url),
                "--start-fullscreen",
            ])
            .spawn()
            .is_err()
        {
            // Fallback to default browser
            let _ = Command::new("cmd").args(["/c", "start", url]).spawn();
        }
    }

    #[cfg(target_os = "linux")]
    {
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
