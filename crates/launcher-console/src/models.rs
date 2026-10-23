use launcher_core::models::{Language, LaunchBehavior};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleSettings {
    /// Active language (Default: English, switchable to Portuguese)
    #[serde(default)]
    pub language: Language,
    /// Comportamento de inicialização de jogos:
    /// Oculta e restaura com foco imediato (padrão) ou Mantém aberto
    pub launch_behavior: LaunchBehavior,
    /// Dispositivo de áudio preferencial (ex.: HDMI TV ou Soundbar)
    #[serde(default)]
    pub audio_device: Option<String>,
    /// Se a extensão DualSense deve iniciar automaticamente junto com a aplicação (Padrão: false)
    #[serde(default)]
    pub dualsense_extension_enabled: bool,
    /// DualSenseX Link UDP server enabled (port 6969, Padrão: false)
    #[serde(default)]
    pub dualsense_udp_enabled: bool,
    /// Pastas configuradas pelo usuário para busca de jogos e ROMs
    #[serde(default)]
    pub scan_directories: Vec<String>,
    /// Caminho personalizado da Steam ou do arquivo shortcuts.vdf apontado pelo usuário
    #[serde(default)]
    pub steam_shortcuts_path: Option<String>,
    /// Caminhos de executáveis de emuladores configurados pelo usuário (ex: "pcsx2" -> "C:\...\pcsx2-qt.exe")
    #[serde(default)]
    pub emulator_paths: std::collections::HashMap<String, String>,
}

impl Default for ConsoleSettings {
    fn default() -> Self {
        Self {
            language: Language::En,
            launch_behavior: LaunchBehavior::HideAndRestoreFullscreen,
            audio_device: None,
            dualsense_extension_enabled: false,
            dualsense_udp_enabled: false,
            scan_directories: vec![dirs::home_dir()
                .map(|h| h.join("Games").to_string_lossy().to_string())
                .unwrap_or_else(|| "./games".to_string())],
            steam_shortcuts_path: None,
            emulator_paths: std::collections::HashMap::new(),
        }
    }
}
