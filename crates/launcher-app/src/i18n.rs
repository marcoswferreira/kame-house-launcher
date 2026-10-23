use launcher_core::models::{Language, LaunchBehavior};

/// Central Internationalization engine for Steam Companion.
/// Default: English (en) with full Portuguese (pt-BR) support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I18n {
    pub language: Language,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            language: Language::En,
        }
    }
}

impl I18n {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn starting_banner(&self, version: &str) -> String {
        match self.language {
            Language::En => format!("Starting Steam Companion v{}...", version),
            Language::PtBr => format!("Iniciando Steam Companion v{}...", version),
        }
    }

    pub fn games_loaded(&self, count: usize) -> String {
        match self.language {
            Language::En => format!("Steam Companion loaded with {} games in catalog.", count),
            Language::PtBr => format!("Steam Companion carregado com {} jogos no catálogo.", count),
        }
    }

    pub fn dualsense_and_console_ready(&self) -> &'static str {
        match self.language {
            Language::En => "DualSense control plane and Console Mode Session ready.",
            Language::PtBr => "Plano de controle DualSense e Sessão Modo Console prontos.",
        }
    }

    pub fn active_language_msg(&self) -> String {
        match self.language {
            Language::En => format!(
                "Active language: {} [{}]",
                self.language.display_name(),
                self.language.code()
            ),
            Language::PtBr => format!(
                "Idioma ativo: {} [{}]",
                self.language.display_name(),
                self.language.code()
            ),
        }
    }

    pub fn launch_behavior_desc(&self, behavior: LaunchBehavior) -> &'static str {
        match self.language {
            Language::En => match behavior {
                LaunchBehavior::HideAndRestoreFullscreen => {
                    "Hide on launch and restore in fullscreen with focus on exit (Console Default)"
                }
                LaunchBehavior::KeepOpen => "Keep launcher open in background",
            },
            Language::PtBr => match behavior {
                LaunchBehavior::HideAndRestoreFullscreen => {
                    "Ocultar ao abrir e restaurar em tela cheia com foco no fechamento (Padrão Console)"
                }
                LaunchBehavior::KeepOpen => "Manter o launcher aberto em segundo plano",
            },
        }
    }

    pub fn launch_behavior_configured(&self, behavior: LaunchBehavior) -> String {
        let desc = self.launch_behavior_desc(behavior);
        match self.language {
            Language::En => format!("Configured launch behavior: {}", desc),
            Language::PtBr => format!("Comportamento de inicialização configurado: {}", desc),
        }
    }

    pub fn cli_help(&self) -> String {
        match self.language {
            Language::En => [
                "Steam Companion - Living Room Console Utility & Controller Hub",
                "",
                "USAGE:",
                "    kame-house-launcher [OPTIONS] [COMMAND]",
                "",
                "OPTIONS:",
                "    -l, --lang <en|pt-br>        Override language for this run (default: en)",
                "        --set-lang <en|pt-br>    Set and persist preferred language",
                "    -h, --help                   Print help information",
                "    -v, --version                Print version information",
                "",
                "COMMANDS:",
                "    status                       Show launcher status, catalog, and configuration",
                "    scan [PATH...]               Scan directories for games and update catalog",
                "    export-steam [PATH]          Export shortcuts into Valve binary shortcuts.vdf",
                "    set-mode <hide|keep-open>    Set game launch behavior (hide & restore vs keep open)",
                "",
                "ENVIRONMENT:",
                "    KAME_HOUSE_LANG              Override default language via environment (e.g., 'en' or 'pt-BR')",
            ]
            .join("\n"),
            Language::PtBr => [
                "Steam Companion - Utilitário Gamer de Sala de Estar e Hub de Controles",
                "",
                "USO:",
                "    kame-house-launcher [OPÇÕES] [COMANDO]",
                "",
                "OPÇÕES:",
                "    -l, --lang <en|pt-br>        Define o idioma para esta execução (padrão: en)",
                "        --set-lang <en|pt-br>    Define e salva o idioma preferencial permanentemente",
                "    -h, --help                   Exibe informações de ajuda",
                "    -v, --version                Exibe a versão do aplicativo",
                "",
                "COMANDOS:",
                "    status                       Exibe status do launcher, catálogo e configurações",
                "    scan [DIRETÓRIO...]          Varre diretórios procurando jogos e atualiza catálogo",
                "    export-steam [CAMINHO]       Exporta atalhos para o shortcuts.vdf binário da Steam",
                "    set-mode <hide|keep-open>    Define o comportamento de lançamento (ocultar vs manter aberto)",
                "",
                "VARIÁVEIS DE AMBIENTE:",
                "    KAME_HOUSE_LANG              Sobrescreve o idioma padrão via ambiente (ex: 'en' ou 'pt-BR')",
            ]
            .join("\n"),
        }
    }

    pub fn scan_completed(&self, count: usize) -> String {
        match self.language {
            Language::En => format!("Scan completed: found and registered {} game(s).", count),
            Language::PtBr => format!(
                "Varredura concluída: {} jogo(s) encontrado(s) e registrado(s).",
                count
            ),
        }
    }

    pub fn export_steam_completed(&self, count: usize, path: &str) -> String {
        match self.language {
            Language::En => format!(
                "Successfully exported {} shortcut(s) to Steam VDF: {}",
                count, path
            ),
            Language::PtBr => format!(
                "{} atalho(s) exportado(s) com sucesso para o VDF da Steam: {}",
                count, path
            ),
        }
    }

    pub fn dualsense_profile_applied(&self, name: &str) -> String {
        match self.language {
            Language::En => format!("Applied DualSense trigger profile: {}", name),
            Language::PtBr => format!("Perfil de gatilhos DualSense aplicado: {}", name),
        }
    }

    pub fn language_updated(&self, lang: Language) -> String {
        match self.language {
            Language::En => format!(
                "Preferred language successfully updated to: {} [{}]",
                lang.display_name(),
                lang.code()
            ),
            Language::PtBr => format!(
                "Idioma de preferência atualizado com sucesso para: {} [{}]",
                lang.display_name(),
                lang.code()
            ),
        }
    }

    pub fn launch_mode_updated(&self, behavior: LaunchBehavior) -> String {
        let desc = self.launch_behavior_desc(behavior);
        match self.language {
            Language::En => format!("Launch mode successfully updated to: {}", desc),
            Language::PtBr => format!(
                "Modo de inicialização atualizado com sucesso para: {}",
                desc
            ),
        }
    }

    pub fn status_title(&self) -> &'static str {
        match self.language {
            Language::En => "=== Steam Companion Status ===",
            Language::PtBr => "=== Status do Steam Companion ===",
        }
    }

    pub fn status_language_label(&self) -> &'static str {
        match self.language {
            Language::En => "Active Language",
            Language::PtBr => "Idioma Ativo",
        }
    }

    pub fn status_games_label(&self) -> &'static str {
        match self.language {
            Language::En => "Catalog Games",
            Language::PtBr => "Jogos no Catálogo",
        }
    }

    pub fn status_launch_behavior_label(&self) -> &'static str {
        match self.language {
            Language::En => "Launch Behavior",
            Language::PtBr => "Comportamento de Execução",
        }
    }

    pub fn status_dualsense_label(&self) -> &'static str {
        match self.language {
            Language::En => "DualSense Control Plane",
            Language::PtBr => "Plano de Controle DualSense",
        }
    }

    pub fn daemon_running_prompt(&self) -> &'static str {
        match self.language {
            Language::En => "🎮  Steam Companion is running. Press Ctrl+C to stop.",
            Language::PtBr => {
                "🎮  O serviço do Steam Companion está em execução. Pressione Ctrl+C para encerrar."
            }
        }
    }

    pub fn daemon_shutting_down(&self) -> &'static str {
        match self.language {
            Language::En => "Shutting down Steam Companion gracefully...",
            Language::PtBr => "Encerrando o Steam Companion com segurança...",
        }
    }
}
