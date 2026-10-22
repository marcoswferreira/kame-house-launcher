use launcher_core::models::LaunchBehavior;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TvResolution {
    Res4k120,
    Res4k60,
    Res1440p120,
    Res1080p120,
    Res1080p60,
}

impl TvResolution {
    pub fn dimensions(&self) -> (u32, u32, u32) {
        match self {
            TvResolution::Res4k120 => (3840, 2160, 120),
            TvResolution::Res4k60 => (3840, 2160, 60),
            TvResolution::Res1440p120 => (2560, 1440, 120),
            TvResolution::Res1080p120 => (1920, 1080, 120),
            TvResolution::Res1080p60 => (1920, 1080, 60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleSettings {
    /// Comportamento de inicialização de jogos:
    /// Oculta e restaura com foco imediato (padrão) ou Mantém aberto
    pub launch_behavior: LaunchBehavior,
    /// Inicializar com o sistema operacional
    pub autostart: bool,
    /// Modo Kiosk / Shell (ocultar barra de tarefas e ícones de desktop)
    pub shell_mode: bool,
    /// Login automático sem senha no usuário gamer
    pub auto_login_enabled: bool,
    /// Resolução e taxa de atualização para TV da sala
    pub tv_resolution: Option<TvResolution>,
    /// Ativação de HDR em TVs compatíveis
    pub hdr_enabled: bool,
    /// Dispositivo de áudio preferencial (ex.: HDMI TV ou Soundbar)
    pub audio_device: Option<String>,
    /// Ocultar automaticamente o cursor do mouse ao usar controle
    pub hide_mouse_cursor: bool,
    /// Silenciar notificações e ativar Focus Assist
    pub suppress_notifications: bool,
    /// Prevenir suspensão de energia durante jogos ou streaming
    pub prevent_sleep: bool,
    /// Wake-on-LAN para ligar o PC via Moonlight
    pub wake_on_lan_enabled: bool,
}

impl Default for ConsoleSettings {
    fn default() -> Self {
        Self {
            launch_behavior: LaunchBehavior::HideAndRestoreFullscreen,
            autostart: true,
            shell_mode: false,
            auto_login_enabled: false,
            tv_resolution: Some(TvResolution::Res4k120),
            hdr_enabled: true,
            audio_device: None,
            hide_mouse_cursor: true,
            suppress_notifications: true,
            prevent_sleep: true,
            wake_on_lan_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerAction {
    Sleep,
    Hibernate,
    Restart,
    Shutdown,
}

