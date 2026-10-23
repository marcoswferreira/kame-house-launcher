pub mod cli;
pub mod i18n;
pub mod server;
pub mod service;
pub mod tray;

pub use i18n::I18n;
pub use service::LauncherService;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{execute_cli, CliArgs, CliCommand};
    use launcher_console::{ConsoleSettings, WindowState};
    use launcher_core::models::{Game, Language, LaunchBehavior};
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn test_language_defaults_and_parsing() {
        assert_eq!(Language::default(), Language::En);
        assert_eq!(Language::En.code(), "en");
        assert_eq!(Language::PtBr.code(), "pt-BR");

        assert_eq!(Language::parse_flexible("en"), Some(Language::En));
        assert_eq!(Language::parse_flexible("en-US"), Some(Language::En));
        assert_eq!(Language::parse_flexible("ENGLISH"), Some(Language::En));
        assert_eq!(Language::parse_flexible("pt"), Some(Language::PtBr));
        assert_eq!(Language::parse_flexible("pt-BR"), Some(Language::PtBr));
        assert_eq!(Language::parse_flexible("pt_br"), Some(Language::PtBr));
        assert_eq!(Language::parse_flexible("portugues"), Some(Language::PtBr));
        assert_eq!(Language::parse_flexible("invalid"), None);
    }

    #[test]
    fn test_i18n_messages_en_and_pt() {
        let i18n_en = I18n::new(Language::En);
        assert!(i18n_en
            .starting_banner("0.1.0")
            .contains("Starting Kame House Launcher"));
        assert!(i18n_en.games_loaded(5).contains("5 games"));
        assert!(i18n_en
            .dualsense_and_console_ready()
            .contains("DualSense control plane"));
        assert!(i18n_en.active_language_msg().contains("English [en]"));
        assert!(i18n_en
            .launch_behavior_desc(LaunchBehavior::HideAndRestoreFullscreen)
            .contains("Hide on launch"));
        assert!(i18n_en
            .launch_behavior_desc(LaunchBehavior::KeepOpen)
            .contains("Keep launcher open"));
        assert!(i18n_en.cli_help().contains("USAGE:"));

        let i18n_pt = I18n::new(Language::PtBr);
        assert!(i18n_pt
            .starting_banner("0.1.0")
            .contains("Iniciando Kame House Launcher"));
        assert!(i18n_pt.games_loaded(5).contains("5 jogos"));
        assert!(i18n_pt
            .dualsense_and_console_ready()
            .contains("Plano de controle DualSense"));
        assert!(i18n_pt.active_language_msg().contains("Português"));
        assert!(i18n_pt
            .launch_behavior_desc(LaunchBehavior::HideAndRestoreFullscreen)
            .contains("Ocultar ao abrir"));
        assert!(i18n_pt
            .launch_behavior_desc(LaunchBehavior::KeepOpen)
            .contains("segundo plano"));
        assert!(i18n_pt.cli_help().contains("USO:"));
    }

    #[test]
    fn test_cli_args_parsing() {
        let help = CliArgs::parse_from(vec!["kame-house-launcher", "--help"]).unwrap();
        assert_eq!(help.command, CliCommand::PrintHelp);

        let ver = CliArgs::parse_from(vec!["kame-house-launcher", "-v"]).unwrap();
        assert_eq!(ver.command, CliCommand::PrintVersion);

        let status =
            CliArgs::parse_from(vec!["kame-house-launcher", "-l", "pt-br", "status"]).unwrap();
        assert_eq!(status.language_override, Some(Language::PtBr));
        assert_eq!(status.command, CliCommand::Status);

        let set_lang =
            CliArgs::parse_from(vec!["kame-house-launcher", "--set-lang", "en"]).unwrap();
        assert_eq!(
            set_lang.command,
            CliCommand::SetLanguage { lang: Language::En }
        );

        let scan = CliArgs::parse_from(vec!["kame-house-launcher", "scan", "/roms/gba"]).unwrap();
        match scan.command {
            CliCommand::Scan { paths } => assert_eq!(paths.len(), 1),
            _ => panic!("Expected Scan command"),
        }

        let mode =
            CliArgs::parse_from(vec!["kame-house-launcher", "set-mode", "keep-open"]).unwrap();
        assert_eq!(
            mode.command,
            CliCommand::SetLaunchMode {
                behavior: LaunchBehavior::KeepOpen
            }
        );

        let invalid = CliArgs::parse_from(vec!["kame-house-launcher", "--lang"]);
        assert!(invalid.is_err());

        let once = CliArgs::parse_from(vec!["kame-house-launcher", "--once"]).unwrap();
        assert_eq!(
            once.command,
            CliCommand::RunDaemon {
                keep_alive: false,
                open_ui: false
            }
        );
    }

    #[tokio::test]
    async fn test_cli_execution_and_language_switching() {
        let dir = tempdir().unwrap();
        let svc = Arc::new(LauncherService::new(dir.path().to_path_buf()));

        // Por padrão o idioma é Inglês
        assert_eq!(svc.language(), Language::En);

        // Trocar para Português via CLI
        let set_pt = CliArgs {
            language_override: None,
            command: CliCommand::SetLanguage {
                lang: Language::PtBr,
            },
        };
        execute_cli(Arc::clone(&svc), set_pt).await.unwrap();
        assert_eq!(svc.language(), Language::PtBr);

        // Trocar de volta para Inglês
        let set_en = CliArgs {
            language_override: None,
            command: CliCommand::SetLanguage { lang: Language::En },
        };
        execute_cli(Arc::clone(&svc), set_en).await.unwrap();
        assert_eq!(svc.language(), Language::En);

        // Status execution
        let status = CliArgs {
            language_override: Some(Language::PtBr),
            command: CliCommand::Status,
        };
        assert!(execute_cli(Arc::clone(&svc), status).await.is_ok());

        // Mode switch execution
        let mode = CliArgs {
            language_override: None,
            command: CliCommand::SetLaunchMode {
                behavior: LaunchBehavior::KeepOpen,
            },
        };
        assert!(execute_cli(Arc::clone(&svc), mode).await.is_ok());
        assert_eq!(
            svc.settings.read().launch_behavior,
            LaunchBehavior::KeepOpen
        );

        // RunDaemon execution with keep_alive = false (once mode)
        let daemon = CliArgs {
            language_override: None,
            command: CliCommand::RunDaemon {
                keep_alive: false,
                open_ui: false,
            },
        };
        assert!(execute_cli(Arc::clone(&svc), daemon).await.is_ok());
    }

    #[tokio::test]
    async fn test_server_static_and_status_routes() {
        let dir = tempdir().unwrap();
        let svc = Arc::new(LauncherService::new(dir.path().to_path_buf()));

        // 1. Test static HTML response
        let (client, server) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server, svc_clone).await;
        });

        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut client = client;
        client.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();
        let mut response = Vec::new();
        let mut buf = [0u8; 1024];
        let n = client.read(&mut buf).await.unwrap();
        response.extend_from_slice(&buf[..n]);

        let resp_str = String::from_utf8_lossy(&response);
        assert!(resp_str.contains("200 OK"));
        assert!(resp_str.contains("Kame House Launcher"));

        // 2. Test CSS route
        let (mut client2, server2) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server2, svc_clone).await;
        });
        client2
            .write_all(b"GET /style.css HTTP/1.1\r\n\r\n")
            .await
            .unwrap();
        let mut resp2 = [0u8; 512];
        let n2 = client2.read(&mut resp2).await.unwrap();
        assert!(String::from_utf8_lossy(&resp2[..n2]).contains("200 OK"));

        // 3. Test API status route
        let (mut client3, server3) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server3, svc_clone).await;
        });
        client3
            .write_all(b"GET /api/status HTTP/1.1\r\n\r\n")
            .await
            .unwrap();
        let mut resp3 = [0u8; 512];
        let n3 = client3.read(&mut resp3).await.unwrap();
        let resp3_str = String::from_utf8_lossy(&resp3[..n3]);
        assert!(resp3_str.contains("200 OK"));
        assert!(resp3_str.contains("games_count"));

        // 4. Test API steam sync route
        let (mut client4, server4) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server4, svc_clone).await;
        });
        client4
            .write_all(b"POST /api/steam/sync HTTP/1.1\r\n\r\n")
            .await
            .unwrap();
        let mut resp4 = [0u8; 512];
        let n4 = client4.read(&mut resp4).await.unwrap();
        assert!(String::from_utf8_lossy(&resp4[..n4]).contains("200 OK"));

        // 5. Test API settings route
        let (mut client5, server5) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server5, svc_clone).await;
        });
        client5
            .write_all(b"POST /api/settings HTTP/1.1\r\n\r\n{\"language\":\"pt-BR\"}")
            .await
            .unwrap();
        let mut resp5 = [0u8; 512];
        let n5 = client5.read(&mut resp5).await.unwrap();
        assert!(String::from_utf8_lossy(&resp5[..n5]).contains("200 OK"));
        assert_eq!(svc.language(), Language::PtBr);

        // 6. Test API window toggle-console & minimize routes
        let (mut client6, server6) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server6, svc_clone).await;
        });
        client6
            .write_all(b"POST /api/window/toggle-console HTTP/1.1\r\n\r\n")
            .await
            .unwrap();
        let mut resp6 = [0u8; 512];
        let n6 = client6.read(&mut resp6).await.unwrap();
        assert!(String::from_utf8_lossy(&resp6[..n6]).contains("\"debug_console_visible\":true"));

        let (mut client7, server7) = tokio::io::duplex(4096);
        let svc_clone = Arc::clone(&svc);
        tokio::spawn(async move {
            let _ = crate::server::handle_connection(server7, svc_clone).await;
        });
        client7
            .write_all(b"POST /api/window/minimize HTTP/1.1\r\n\r\n")
            .await
            .unwrap();
        let mut resp7 = [0u8; 512];
        let n7 = client7.read(&mut resp7).await.unwrap();
        assert!(String::from_utf8_lossy(&resp7[..n7]).contains("minimized_to_tray"));
    }

    #[test]
    fn test_launcher_service_flow() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf());

        let game1 = Game::new(
            "Metroid Fusion",
            "/roms/fusion.gba",
            "gba",
            "Game Boy Advance",
        );
        svc.add_game(game1.clone());

        let games = svc.list_games();
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].title, "Metroid Fusion");

        // Testar prepare launch com o comportamento padrão (HideAndRestoreFullscreen)
        let (launched, state_before) = svc
            .prepare_launch(&game1.id)
            .expect("Jogo deve ser preparado");
        assert_eq!(launched.title, "Metroid Fusion");
        assert_eq!(state_before, WindowState::MinimizedOrHidden);

        let state_after = svc.finish_launch(&game1.id, 120);
        assert_eq!(state_after, WindowState::FullscreenFocused);

        let updated_games = svc.list_games();
        assert_eq!(updated_games[0].playtime_seconds, 120);
        assert!(updated_games[0].last_played_at.is_some());
    }

    #[test]
    fn test_launcher_service_keep_open_mode() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf());

        let settings = ConsoleSettings {
            launch_behavior: LaunchBehavior::KeepOpen,
            ..Default::default()
        };
        svc.update_console_settings(settings);

        let game = Game::new("Sonic 2", "/roms/sonic2.md", "genesis", "Mega Drive");
        svc.add_game(game.clone());

        let (_, state_before) = svc.prepare_launch(&game.id).unwrap();
        assert_eq!(state_before, WindowState::Normal);

        let state_after = svc.finish_launch(&game.id, 60);
        assert_eq!(state_after, WindowState::Normal);
    }

    #[test]
    fn test_steam_export_from_service() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf());

        let game = Game::new("God of War Ghost of Sparta", "/roms/gow.cso", "psp", "PSP");
        svc.add_game(game);

        let vdf_path = dir.path().join("shortcuts.vdf");
        let exported_count = svc.export_games_to_steam(&vdf_path).unwrap();
        assert_eq!(exported_count, 1);
        assert!(vdf_path.exists());
    }

    #[tokio::test]
    async fn test_dualsense_integration_from_service() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf());

        let status = svc.dualsense.get_status().await;
        assert!(!status.extension_enabled);
        assert!(!status.autostart_on_launch);
        assert!(!status.connected);
        assert_eq!(status.battery.percentage, 0);
        assert!(!status.profiles.is_empty());

        svc.set_dualsense_extension_active(true);
        svc.set_dualsense_autostart(true);
        assert!(svc.dualsense.is_extension_enabled());
        assert!(svc.dualsense.is_autostart_enabled());

        let applied = svc.dualsense.apply_profile("racing_abs");
        assert!(applied);

        let updated = svc.dualsense.get_status().await;
        assert_eq!(
            updated.lighting.lightbar,
            launcher_dualsense::LightbarMode::Static {
                color: launcher_dualsense::RgbColor::RED
            }
        );

        svc.set_dualsense_extension_active(false);
        assert!(!svc.dualsense.is_extension_enabled());
    }

    #[tokio::test]
    async fn test_server_bind_with_fallback() {
        let dir = tempdir().unwrap();
        let svc = Arc::new(LauncherService::new(dir.path().to_path_buf()));

        // 1. Basic bind with ephemeral fallback
        let (_server, listener, bound_port) =
            crate::server::HttpServer::bind_with_fallback(Arc::clone(&svc), 0)
                .await
                .unwrap();
        assert!(bound_port > 0);
        drop(listener);

        // 2. Fallback when preferred port is occupied
        // Bind an arbitrary port first to simulate conflict
        let conflict_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let conflict_port = conflict_listener.local_addr().unwrap().port();

        let (_server2, listener2, bound_port2) =
            crate::server::HttpServer::bind_with_fallback(Arc::clone(&svc), conflict_port)
                .await
                .unwrap();
        assert_ne!(bound_port2, conflict_port);
        assert!(bound_port2 > 0);
        drop(listener2);
        drop(conflict_listener);
    }
}
