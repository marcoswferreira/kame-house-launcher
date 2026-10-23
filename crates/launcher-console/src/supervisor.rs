use crate::models::ConsoleSettings;
use launcher_core::models::LaunchBehavior;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    MinimizedOrHidden,
    FullscreenFocused,
}

pub struct LaunchSupervisor {
    pub settings: ConsoleSettings,
    pub current_window_state: WindowState,
}

impl LaunchSupervisor {
    pub fn new(settings: ConsoleSettings) -> Self {
        Self {
            settings,
            current_window_state: WindowState::FullscreenFocused,
        }
    }

    /// Determina a transição da janela antes do jogo abrir
    pub fn on_game_launch_prepare(&mut self) -> WindowState {
        match self.settings.launch_behavior {
            LaunchBehavior::HideAndRestoreFullscreen => {
                self.current_window_state = WindowState::MinimizedOrHidden;
            }
            LaunchBehavior::KeepOpen => {
                self.current_window_state = WindowState::Normal;
            }
        }
        self.current_window_state
    }

    /// Determina a transição da janela após o jogo fechar
    pub fn on_game_exit(&mut self) -> WindowState {
        match self.settings.launch_behavior {
            LaunchBehavior::HideAndRestoreFullscreen => {
                self.current_window_state = WindowState::FullscreenFocused;
            }
            LaunchBehavior::KeepOpen => {
                self.current_window_state = WindowState::Normal;
            }
        }
        self.current_window_state
    }
}
