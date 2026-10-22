pub mod service;

pub use service::LauncherService;

#[cfg(test)]
mod tests {
    use super::*;
    use launcher_console::{ConsoleSettings, WindowState};
    use launcher_core::models::{Game, LaunchBehavior};
    use tempfile::tempdir;

    #[test]
    fn test_launcher_service_flow() {
        let dir = tempdir().unwrap();
        let svc = LauncherService::new(dir.path().to_path_buf(), "https://127.0.0.1:47990");

        let game1 = Game::new("Metroid Fusion", "/roms/fusion.gba", "gba", "Game Boy Advance");
        svc.add_game(game1.clone());

        let games = svc.list_games();
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].title, "Metroid Fusion");

        // Testar prepare launch com o comportamento padrão (HideAndRestoreFullscreen)
        let (launched, state_before) = svc.prepare_launch(&game1.id).expect("Jogo deve ser preparado");
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

        let mut settings = ConsoleSettings::default();
        settings.launch_behavior = LaunchBehavior::KeepOpen;
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

        let game = Game::new("Zelda Ocarina of Time", "/roms/oot.z64", "n64", "Nintendo 64");
        svc.add_game(game);

        let apps_path = dir.path().join("apps.json");
        let apps_count = svc.sync_sunshine_catalog(&apps_path, "/usr/bin/kame-house-launcher").unwrap();
        // 1 app Big Picture + 1 jogo = 2
        assert_eq!(apps_count, 2);
        assert!(apps_path.exists());
    }
}

