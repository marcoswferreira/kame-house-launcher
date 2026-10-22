pub mod client;
pub mod models;
pub mod sync;

pub use client::SunshineClient;
pub use models::{attribution_notice, PairPinRequest, SunshineApp, SunshineAppsList, SunshineClientInfo, SunshineStatus};
pub use sync::sync_games_to_apps_json;

#[cfg(test)]
mod tests {
    use super::*;
    use launcher_core::models::Game;
    use tempfile::tempdir;

    #[test]
    fn test_attribution_notice() {
        let notice = attribution_notice();
        assert!(notice.contains("LizardByte & Sunshine"));
        assert!(notice.contains("GPL-3.0"));
        assert!(notice.contains("Kame House Launcher"));
    }

    #[test]
    fn test_pair_pin_serialization() {
        let req = PairPinRequest {
            pin: "1234".to_string(),
            name: Some("Moonlight Client".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""pin":"1234""#));
        assert!(json.contains(r#""name":"Moonlight Client""#));
    }

    #[test]
    fn test_sunshine_client_builder() {
        let client = SunshineClient::new("https://127.0.0.1:47990");
        assert_eq!(client.base_url, "https://127.0.0.1:47990");
        assert!(client.csrf_token.is_none());

        let custom = reqwest::Client::new();
        let client2 = SunshineClient::with_client(custom, "https://localhost:47990");
        assert_eq!(client2.base_url, "https://localhost:47990");
    }

    #[test]
    fn test_sync_games_to_apps_json() {
        let dir = tempdir().unwrap();
        let apps_path = dir.path().join("apps.json");

        let mut g_custom = Game::new("Custom Port", "/games/port.exe", "pc", "PC");
        g_custom.custom_exe = Some("/games/launch.exe".to_string());
        g_custom.custom_args = Some("-windowed".to_string());

        let mut g_hidden = Game::new("Hidden Secret", "/games/secret.iso", "ps2", "PS2");
        g_hidden.hidden = true;

        let games = vec![
            Game::new("Super Mario 64", "/games/sm64.z64", "n64", "Nintendo 64"),
            Game::new("Chrono Trigger", "/games/ct.sfc", "snes", "Super Nintendo"),
            g_custom,
            g_hidden,
        ];

        let result = sync_games_to_apps_json(&apps_path, &games, "/opt/kame-house-launcher").unwrap();

        // 1 app principal + 3 jogos visíveis = 4 apps
        assert_eq!(result.apps.len(), 4);
        assert_eq!(result.apps[0].name, "Kame House Launcher (Console)");
        assert_eq!(result.apps[1].name, "Super Mario 64");
        assert_eq!(result.apps[2].name, "Chrono Trigger");
        assert_eq!(result.apps[3].name, "Custom Port");
        assert_eq!(result.apps[3].cmd, Some(r#""/games/launch.exe" -windowed"#.to_string()));
        assert!(apps_path.exists());
    }
}
