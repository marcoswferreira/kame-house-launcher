pub mod audio;
pub mod models;
pub mod supervisor;

pub use audio::{
    execute_set_microphone_mute, execute_toggle_microphone, get_set_mic_mute_command,
    get_switch_playback_device_command, get_toggle_mic_command,
};

pub use models::ConsoleSettings;
pub use supervisor::{LaunchSupervisor, WindowState};

#[cfg(test)]
mod tests {
    use super::*;
    use launcher_core::models::LaunchBehavior;

    #[test]
    fn test_launch_behavior_default_hide_and_restore() {
        let mut settings = ConsoleSettings::default();
        assert_eq!(
            settings.launch_behavior,
            LaunchBehavior::HideAndRestoreFullscreen
        );

        let mut supervisor = LaunchSupervisor::new(settings.clone());
        assert_eq!(
            supervisor.current_window_state,
            WindowState::FullscreenFocused
        );

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
}
