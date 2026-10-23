pub mod cli;
pub mod i18n;
pub mod service;

pub use i18n::I18n;
pub use service::LauncherService;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{execute_cli, CliArgs, CliCommand};
    use launcher_console::{ConsoleSettings, WindowState};
    use launcher_core::models::{Game, Language, LaunchBehavior};
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
            .sunshine_and_console_ready()
            .contains("Sunshine control plane"));
        assert!(i18n_en.active_language_msg().contains("English [en]"));
        assert!(i18n_en
            .launch_behavior_desc(LaunchBehavior::HideAndRestoreFullscreen)
            .contains("Hide on launch"));
        assert!(i18n_en
            .launch_behavior_desc(LaunchBehavior::KeepOpen)
            .contains("Keep launcher open"));
        assert_eq!(i18n_en.power_action_name("sleep"), "Sleep");
        assert!(i18n_en.cli_help().contains("USAGE:"));

        let i18n_pt = I18n::new(Language::PtBr);
        assert!(i18n_pt
            .starting_banner("0.1.0")
            .contains("Iniciando Kame House Launcher"));
        assert!(i18n_pt.games_loaded(5).contains("5 jogos"));
        assert!(i18n_pt
            .sunshine_and_console_ready()
            .contains("Plano de controle Sunshine"));
        assert!(i18n_pt.active_language_msg().contains("Português"));
        assert!(i18n_pt
            .launch_behavior_desc(LaunchBehavior::HideAndRestoreFullscreen)
            .contains("Ocultar ao abrir"));
        assert!(i18n_pt
            .launch_behavior_desc(LaunchBehavior::KeepOpen)
            .contains("segundo plano"));
        assert_eq!(i18n_pt.power_action_name("sleep"), "Suspender");
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
    }

    #[test]
    fn test_cli_execution_and_language_switching() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

        // Por padrão o idioma é Inglês
        assert_eq!(svc.language(), Language::En);

        // Trocar para Português via CLI
        let set_pt = CliArgs {
            language_override: None,
            command: CliCommand::SetLanguage {
                lang: Language::PtBr,
            },
        };
        execute_cli(&svc, set_pt).unwrap();
        assert_eq!(svc.language(), Language::PtBr);

        // Trocar de volta para Inglês
        let set_en = CliArgs {
            language_override: None,
            command: CliCommand::SetLanguage { lang: Language::En },
        };
        execute_cli(&svc, set_en).unwrap();
        assert_eq!(svc.language(), Language::En);

        // Status execution
        let status = CliArgs {
            language_override: Some(Language::PtBr),
            command: CliCommand::Status,
        };
        assert!(execute_cli(&svc, status).is_ok());

        // Mode switch execution
        let mode = CliArgs {
            language_override: None,
            command: CliCommand::SetLaunchMode {
                behavior: LaunchBehavior::KeepOpen,
            },
        };
        assert!(execute_cli(&svc, mode).is_ok());
        assert_eq!(
            svc.settings.read().launch_behavior,
            LaunchBehavior::KeepOpen
        );
    }

    #[test]
    fn test_launcher_service_flow() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

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
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

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
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

        let game = Game::new("God of War Ghost of Sparta", "/roms/gow.cso", "psp", "PSP");
        svc.add_game(game);

        let vdf_path = dir.path().join("shortcuts.vdf");
        let exported_count = svc.export_games_to_steam(&vdf_path).unwrap();
        assert_eq!(exported_count, 1);
        assert!(vdf_path.exists());
    }

    #[test]
    fn test_sunshine_sync_from_service() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

        let game = Game::new(
            "Zelda Ocarina of Time",
            "/roms/oot.z64",
            "n64",
            "Nintendo 64",
        );
        svc.add_game(game);

        let apps_path = dir.path().join("apps.json");
        let apps_count = svc
            .sync_sunshine_catalog(&apps_path, "/usr/bin/kame-house-launcher")
            .unwrap();
        // 1 app Big Picture + 1 jogo = 2
        assert_eq!(apps_count, 2);
        assert!(apps_path.exists());
    }
}
