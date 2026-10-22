pub mod boot;
pub mod models;
pub mod power;
pub mod supervisor;

pub use boot::{generate_linux_autostart_desktop_file, generate_windows_autologin_reg, save_linux_autostart};
pub use models::{ConsoleSettings, PowerAction, TvResolution};
pub use power::get_power_command;
pub use supervisor::{LaunchSupervisor, WindowState};

#[cfg(test)]
mod tests {
    use super::*;
    use launcher_core::models::LaunchBehavior;
    use tempfile::tempdir;

    #[test]
    fn test_launch_behavior_default_hide_and_restore() {
        let mut settings = ConsoleSettings::default();
        assert_eq!(settings.launch_behavior, LaunchBehavior::HideAndRestoreFullscreen);

        let mut supervisor = LaunchSupervisor::new(settings.clone());
        assert_eq!(supervisor.current_window_state, WindowState::FullscreenFocused);

        // Ao preparar para lançar o jogo, deve ocultar/minimizar
        let state_before = supervisor.on_game_launch_prepare();
        assert_eq!(state_before, WindowState::MinimizedOrHidden);

        // Ao fechar o jogo, deve restaurar em tela cheia com foco imediato
        let state_after = supervisor.on_game_exit();
        assert_eq!(state_after, WindowState::FullscreenFocused);

        // Agora testamos o segundo formato: KeepOpen
        settings.launch_behavior = LaunchBehavior::KeepOpen;
        let mut supervisor2 = LaunchSupervisor::new(settings);

        let state_before2 = supervisor2.on_game_launch_prepare();
        assert_eq!(state_before2, WindowState::Normal);

        let state_after2 = supervisor2.on_game_exit();
        assert_eq!(state_after2, WindowState::Normal);
    }

    #[test]
    fn test_tv_resolutions() {
        let (w, h, hz) = TvResolution::Res4k120.dimensions();
        assert_eq!((w, h, hz), (3840, 2160, 120));

        let (w1080, h1080, hz1080) = TvResolution::Res1080p120.dimensions();
        assert_eq!((w1080, h1080, hz1080), (1920, 1080, 120));
    }

    #[test]
    fn test_autostart_generation() {
        let dir = tempdir().unwrap();
        let saved_path = save_linux_autostart(dir.path(), "/usr/bin/kame-house-launcher").unwrap();
        assert!(saved_path.exists());

        let content = std::fs::read_to_string(&saved_path).unwrap();
        assert!(content.contains("Kame House Launcher"));
        assert!(content.contains("/usr/bin/kame-house-launcher --fullscreen"));
    }

    #[test]
    fn test_windows_autologin_reg() {
        let reg = generate_windows_autologin_reg("Gamer", None);
        assert!(reg.contains(r#""AutoAdminLogon"="1""#));
        assert!(reg.contains(r#""DefaultUserName"="Gamer""#));
    }

    #[test]
    fn test_power_commands() {
        let (cmd, args) = get_power_command(PowerAction::Shutdown);
        assert!(!cmd.is_empty());
        assert!(!args.is_empty());
    }
}

